use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate};
use crate::{
    entities::{expenses, reports},
    services::reports::ReportsService,
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

#[derive(Serialize)]
pub struct CreateExpenseResponse {
    pub expense_id: i32,
    pub description: String,
    pub label: String,
    pub amount: f64,
    pub expense_date: NaiveDate,
    pub created_by: Option<i32>,
    pub modified_by: Option<i32>,
}

#[derive(Serialize)]
pub struct AllExpensesResponse {
    pub expense_id: i32,
    pub description: String,
    pub label: String,
    pub amount: f64,
    pub expense_date: NaiveDate,
    pub created_by: Option<i32>,
    pub modified_by: Option<i32>,
}

#[derive(Serialize)]
pub struct GetExpenseResponse {
    pub expense_id: i32,
    pub description: String,
    pub label: String,
    pub amount: f64,
    pub expense_date: NaiveDate,
    pub created_by: Option<i32>,
    pub modified_by: Option<i32>,
}

#[derive(Serialize)]
pub struct UpdateExpenseResponse {
    pub expense_id: i32,
    pub label: String,
    pub order_date: NaiveDate,
    pub total_amount: f64,
    pub created_by: Option<i32>,
    pub modified_by: Option<i32>,
}

impl ExpensesService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Create a new expense
    pub async fn create_expense(&self, req: CreateExpenseRequest) -> Result<CreateExpenseResponse, AppError> {
        let new_expense = expenses::ActiveModel {
            description: Set(req.description.clone()),
            label: Set(req.label.clone()),
            amount: Set(req.amount),
            expense_date: Set(req.expense_date),
            created_by: Set(Some(req.created_by)),
            modified_by: Set(Some(req.created_by)),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        // After saving the expense
        let expense_date = new_expense.expense_date;
        let first_day_of_month = NaiveDate::from_ymd_opt(expense_date.year(), expense_date.month(), 1)
            .expect("Invalid expense date");

        let reports_service = ReportsService::new(self.db.clone());
        if let Err(e) = reports_service.generate_monthly_report(first_day_of_month).await {
            tracing::error!("Failed to auto-update monthly report after expense creation: {}", e);
        }

        Ok(CreateExpenseResponse {
            expense_id: new_expense.expense_id,
            description: new_expense.description,
            label: new_expense.label,
            amount: new_expense.amount,
            expense_date: new_expense.expense_date,
            created_by: new_expense.created_by,
            modified_by: new_expense.modified_by,
        })
    }

    /// Fetch all expenses
    pub async fn get_expenses(&self) -> Result<Vec<AllExpensesResponse>, AppError> {
        let all_expenses = expenses::Entity::find()
            .all(&self.db)
            .await
            .map_err(AppError::from)?;

        let response = all_expenses
            .into_iter()
            .map(|all_expenses| AllExpensesResponse {
                expense_id: all_expenses.expense_id,
                description: all_expenses.description,
                label: all_expenses.label,
                amount: all_expenses.amount,
                expense_date: all_expenses.expense_date,
                created_by: all_expenses.created_by,
                modified_by: all_expenses.modified_by,
            }).collect();

        Ok(response)
    }

    /// Fetch single expense by ID
    pub async fn get_expense_by_id(&self, expense_id: i32) -> Result<GetExpenseResponse, AppError> {
        let expense = expenses::Entity::find_by_id(expense_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Order not found".into()))?;

        Ok(GetExpenseResponse {
            expense_id: expense.expense_id,
            description: expense.description,
            label: expense.label,
            amount: expense.amount,
            expense_date: expense.expense_date,
            created_by: expense.created_by,
            modified_by: expense.modified_by,
        })
    }

    /// Update expense
    pub async fn update_expense(&self, expense_id: i32, req: UpdateExpenseRequest, modified_by: i32) -> Result<UpdateExpenseResponse, AppError> {
        use expenses::Entity as Expenses;

        // Find existing expense
        let existing = Expenses::find_by_id(expense_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Expense not found".into()))?;

        // Convert to active model
        let mut active: expenses::ActiveModel = existing.into();

        if let Some(desc) = req.description {
            active.description = Set(desc);
        }

        if let Some(label) = req.label {
            active.label = Set(label);
        }

        if let Some(amount) = req.amount {
            active.amount = Set(amount);
        }

        if let Some(date) = req.expense_date {
            active.expense_date = Set(date);
        }

        active.modified_by = Set(Some(modified_by));

        // Update in DB
        let updated = active.update(&self.db).await?;

        // After updating expense
        if let Some(updated) = updated {
            let expense_date = updated.expense_date;
            let first_day_of_month = NaiveDate::from_ymd_opt(expense_date.year(), expense_date.month(), 1)
                .expect("Invalid expense date");

            let reports_service = ReportsService::new(self.db.clone());
            if let Err(e) = reports_service.generate_monthly_report(first_day_of_month).await {
                tracing::error!("Failed to auto-update monthly report after expense update: {}", e);
            }
        }

        Ok(UpdateExpenseResponse {
            expense_id: updated.expense_id,
            description: updated.description,
            label: updated.label,
            amount: updated.amount,
            expense_date: updated.expense_date,
            created_by: updated.created_by,
            modified_by: updated.modified_by,
        })
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
