use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use chrono::NaiveDate;
use serde_json::json;

use crate::{
    middleware::auth::AuthenticatedUser,
    services::orders::{OrdersService, CreateOrderRequest as ServiceCreateRequest, UpdateOrderRequest as ServiceUpdateRequest},
    errors::AppError,
    services::invoices::InvoicesService,
};

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub patient_name: String,
    pub order_date: String,      // YYYY-MM-DD
    pub total_amount: f64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderRequest {
    pub patient_name: Option<String>,
    pub order_date: Option<String>, // YYYY-MM-DD
    pub total_amount: Option<f64>,
    pub description: Option<String>,
}

/// POST /orders
pub async fn create_order(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    payload: web::Json<CreateOrderRequest>,
) -> Result<HttpResponse, AppError> {
    let service = OrdersService::new(db.get_ref().clone());

    let req = ServiceCreateRequest {
        patient_name: payload.patient_name.clone(),
        order_date: NaiveDate::parse_from_str(&payload.order_date, "%Y-%m-%d")
            .map_err(|_| AppError::BadRequest("Invalid date format, expected YYYY-MM-DD".into()))?,
        total_amount: payload.total_amount,
        description: payload.description.clone(),
        created_by: user.user_id,
    };

    // Create order and auto-generate invoice
    let (order, invoice) = service.create_order(req).await?;

    // Return both order and invoice in JSON response
    Ok(HttpResponse::Ok().json(json!({
        "order": order,
        "invoice": invoice
    })))
}


/// GET /orders
pub async fn list_orders(
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, AppError> {
    let service = OrdersService::new(db.get_ref().clone());
    let result = service.get_orders().await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /orders/{id}
pub async fn get_order(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let service = OrdersService::new(db.get_ref().clone());
    let order = service.get_order_by_id(id).await?;
    Ok(HttpResponse::Ok().json(order))
}

/// PUT /orders/{id}
pub async fn update_order(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
    payload: web::Json<UpdateOrderRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let service = OrdersService::new(db.get_ref().clone());

    let req = ServiceUpdateRequest {
        patient_name: payload.patient_name.clone(),
        order_date: match &payload.order_date {
            Some(d) => Some(
                NaiveDate::parse_from_str(d, "%Y-%m-%d")
                    .map_err(|_| AppError::BadRequest("Invalid date format, expected YYYY-MM-DD".into()))?,
            ),
            None => None,
        },
        total_amount: payload.total_amount,
        description: payload.description.clone(),
    };

    let (updated_order, updated_invoice) = service.update_order(id, req, user.user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "order": updated_order,
        "invoice": updated_invoice
    })))
}

/// DELETE /orders/{id}
pub async fn delete_order(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let service = OrdersService::new(db.get_ref().clone());
    service.delete_order(id).await?;
    Ok(HttpResponse::Ok().json("Order deleted successfully"))
}