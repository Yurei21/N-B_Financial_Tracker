use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    middleware::auth::AuthenticatedUser,
    services::orders,
    errors::AppError,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub patient_name: String,
    pub order_date: String,      
    pub total_amount: f64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderRequest {
    pub patient_name: Option<String>,
    pub order_date: Option<String>,
    pub total_amount: Option<f64>,
    pub description: Option<String>,
}

/// POST /orders
/// Create a new order (auto-links to logged-in user)
pub async fn create_order(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    payload: web::Json<CreateOrderRequest>,
) -> Result<HttpResponse, AppError> {
    let order = orders::create_order(&db, user.user_id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(order))
}

/// GET /orders
/// Fetch all orders for the logged-in user
pub async fn list_orders(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let result = orders::list_orders(&db, user.user_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /orders/{id}
/// Fetch a single order by ID (only if created by user)
pub async fn get_order(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let order = orders::get_order(&db, id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(order))
}

/// PUT /orders/{id}
/// Update an existing order
pub async fn update_order(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
    payload: web::Json<UpdateOrderRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let updated = orders::update_order(&db, id, user.user_id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

/// DELETE /orders/{id}
/// Delete an order (cascade deletes invoice)
pub async fn delete_order(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    orders::delete_order(&db, id, user.user_id).await?;
    Ok(HttpResponse::Ok().json("Order deleted successfully"))
}