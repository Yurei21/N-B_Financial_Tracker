use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    middleware::auth::AuthenticatedUser,
    services::expenses,
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
    pub expense_date: Option<String>,
}

/// POST /expenses
/// Create a new expense entry (auto-linked to the logged-in user)
pub async fn create_expense(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    payload: web::Json<CreateExpenseRequest>,
) -> Result<HttpResponse, AppError> {
    let expense = expenses::create_expense(&db, user.user_id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(expense))
}

/// GET /expenses
/// Fetch all expenses created by the user
pub async fn list_expenses(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let result = expenses::list_expenses(&db, user.user_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /expenses/{id}
/// Fetch a single expense (only if owned by the user)
pub async fn get_expense(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let expense = expenses::get_expense(&db, id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(expense))
}

/// PUT /expenses/{id}
/// Update an existing expense
pub async fn update_expense(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
    payload: web::Json<UpdateExpenseRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let updated = expenses::update_expense(&db, id, user.user_id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

/// DELETE /expenses/{id}
/// Delete a user-owned expense
pub async fn delete_expense(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    expenses::delete_expense(&db, id, user.user_id).await?;
    Ok(HttpResponse::Ok().json("Expense deleted successfully"))
}