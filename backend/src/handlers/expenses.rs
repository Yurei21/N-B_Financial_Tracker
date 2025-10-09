use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use chrono::NaiveDate;

use crate::{
    middleware::auth::AuthenticatedUser,
    services::expenses::{ExpensesService, CreateExpenseRequest as ServiceCreateRequest, UpdateExpenseRequest as ServiceUpdateRequest},
    errors::AppError,
};

#[derive(Debug, Deserialize)]
pub struct CreateExpenseRequest {
    pub description: String,
    pub label: String,
    pub amount: f64,
    pub expense_date: String, // YYYY-MM-DD
}

#[derive(Debug, Deserialize)]
pub struct UpdateExpenseRequest {
    pub description: Option<String>,
    pub label: Option<String>,
    pub amount: Option<f64>,
    pub expense_date: Option<String>, // YYYY-MM-DD
}

/// POST /expenses
pub async fn create_expense(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    payload: web::Json<CreateExpenseRequest>,
) -> Result<HttpResponse, AppError> {
    let service = ExpensesService::new(db.get_ref().clone());

    let req = ServiceCreateRequest {
        description: payload.description.clone(),
        label: payload.label.clone(),
        amount: payload.amount,
        expense_date: NaiveDate::parse_from_str(&payload.expense_date, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Invalid date format, expected YYYY-MM-DD".into()))?,
        created_by: user.user_id,
    };

    let expense = service.create_expense(req).await?;
    Ok(HttpResponse::Ok().json(expense))
}

/// GET /expenses
pub async fn list_expenses(
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, AppError> {
    let service = ExpensesService::new(db.get_ref().clone());
    let result = service.get_expenses().await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /expenses/{id}
pub async fn get_expense(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let service = ExpensesService::new(db.get_ref().clone());
    let expense = service.get_expense_by_id(id).await?;
    Ok(HttpResponse::Ok().json(expense))
}

/// PUT /expenses/{id}
pub async fn update_expense(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
    payload: web::Json<UpdateExpenseRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let service = ExpensesService::new(db.get_ref().clone());

    let req = ServiceUpdateRequest {
        description: payload.description.clone(),
        label: payload.label.clone(),
        amount: payload.amount,
        expense_date: match &payload.expense_date {
            Some(d) => Some(
                NaiveDate::parse_from_str(d, "%Y-%m-%d")
                    .map_err(|_| AppError::BadRequest("Invalid date format, expected YYYY-MM-DD".into()))?,
            ),
            None => None,
        },
    };

    let updated = service.update_expense(id, req, user.user_id).await?;
    Ok(HttpResponse::Ok().json(updated))
}

/// DELETE /expenses/{id}
pub async fn delete_expense(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let service = ExpensesService::new(db.get_ref().clone());
    service.delete_expense(id).await?;
    Ok(HttpResponse::Ok().json("Expense deleted successfully"))
}