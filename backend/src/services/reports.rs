use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Datelike};
use crate::{
    entities::{orders, expenses, reports},
    errors::AppError,
};

#[derive(Clone)]
pub struct ReportsService {
    pub db: DatabaseConnection,
}

#[derive(Serialize, Deserialize)]
pub struct MonthlyReport {
    pub month: NaiveDate,
    pub total_orders: i32,
    pub total_income: f64,
    pub total_expenses: f64,
    pub net_profit: f64,
}

impl ReportsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Helper to get the last day of a month
    fn last_day_of_month(date: NaiveDate) -> NaiveDate {
        let (year, month) = (date.year(), date.month());
        let (next_year, next_month) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
        let first_of_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .expect("Invalid date for next month");
        first_of_next_month.pred_opt().expect("Failed to get last day")
    }

    /// Generate monthly report for a given month (YYYY-MM-01)
    pub async fn generate_monthly_report(&self, month: NaiveDate) -> Result<reports::Model, AppError> {
        let end_of_month = Self::last_day_of_month(month);

        // Orders in that month
        let orders_list = orders::Entity::find()
            .filter(orders::Column::OrderDate.between(month, end_of_month))
            .all(&self.db)
            .await?;

        let total_orders = orders_list.len() as i32;
        let total_income: f64 = orders_list.iter().map(|o| o.total_amount as f64).sum();

        // Expenses in that month
        let expenses_list = expenses::Entity::find()
            .filter(expenses::Column::ExpenseDate.between(month, end_of_month))
            .all(&self.db)
            .await?;

        let total_expenses: f64 = expenses_list.iter().map(|e| e.amount as f64).sum();
        let net_profit = total_income - total_expenses;

        let daily_data = serde_json::json!({
            "orders": orders_list,
            "expenses": expenses_list
        });

        let new_report = reports::ActiveModel {
            month: Set(month),
            total_orders: Set(total_orders),
            total_income: Set(total_income),
            total_expenses: Set(total_expenses),
            net_profit: Set(net_profit),
            daily_data: Set(daily_data),
            ..Default::default()
        };

        let report = new_report.insert(&self.db).await?;
        Ok(report)
    }

    /// Fetch all reports
    pub async fn get_all_reports(&self) -> Result<Vec<reports::Model>, AppError> {
        let all_reports = reports::Entity::find()
            .all(&self.db)
            .await?;
        Ok(all_reports)
    }

    /// Fetch a report by month
    pub async fn get_report_by_month(&self, month: NaiveDate) -> Result<reports::Model, AppError> {
        let report = reports::Entity::find()
            .filter(reports::Column::Month.eq(month))
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(report)
    }
}