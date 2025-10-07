use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::{
    entities::expenses,
    errors::AppError,
};

#[derive(Clone)]
pub struct ExpensesService {
    pub db: DatabaseConnection,
}

#[derive(Deserialize)]
pub struct CreateExpenseRequest {
    pub description: String,
    pub label: String,
    pub amount: f64,
    pub expense_date: NaiveDate,
    pub created_by: i32,
}

#[derive(Deserialize)]
pub struct UpdateExpenseRequest {
    pub description: Option<String>,
    pub label: Option<String>,
    pub amount: Option<f64>,
    pub expense_date: Option<NaiveDate>,
}

impl ExpensesService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Create a new expense
    pub async fn create_expense(&self, req: CreateExpenseRequest) -> Result<expenses::Model, AppError> {
        let new_expense = expenses::ActiveModel {
            description: Set(req.description.clone()),
            label: Set(req.label.clone()),
            amount: Set(req.amount),
            expense_date: Set(req.expense_date),
            created_by: Set(Some(req.created_by)),
            modified_by: Set(Some(req.created_by)),
            ..Default::default()
        };

        let expense = new_expense.insert(&self.db).await?;
        Ok(expense)
    }

    /// Fetch all expenses
    pub async fn get_expenses(&self) -> Result<Vec<expenses::Model>, AppError> {
        let all_expenses = expenses::Entity::find()
            .all(&self.db)
            .await?;
        Ok(all_expenses)
    }

    /// Fetch single expense by ID
    pub async fn get_expense_by_id(&self, expense_id: i32) -> Result<expenses::Model, AppError> {
        let expense = expenses::Entity::find_by_id(expense_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(expense)
    }

    /// Update expense
    pub async fn update_expense(&self, expense_id: i32, req: UpdateExpenseRequest, modified_by: i32) -> Result<expenses::Model, AppError> {
        let mut expense: expenses::ActiveModel = expenses::Entity::find_by_id(expense_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?
            .into();

        if let Some(desc) = req.description { expense.description = Set(desc); }
        if let Some(label) = req.label { expense.label = Set(label); }
        if let Some(amount) = req.amount { expense.amount = Set(amount); }
        if let Some(date) = req.expense_date { expense.expense_date = Set(date); }

        expense.modified_by = Set(Some(modified_by));

        let updated = expense.update(&self.db).await?;
        Ok(updated)
    }

    /// Delete expense
    pub async fn delete_expense(&self, expense_id: i32) -> Result<(), AppError> {
        let expense: expenses::ActiveModel = expenses::Entity::find_by_id(expense_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?
            .into();

        expense.delete(&self.db).await?;
        Ok(())
    }
}
