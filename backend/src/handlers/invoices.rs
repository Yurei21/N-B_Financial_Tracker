use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    middleware::auth::AuthenticatedUser,
    services::invoices::{InvoicesService, CreateInvoiceRequest as ServiceCreateRequest},
    errors::AppError,
};

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceRequest {
    pub order_id: i32,
    pub transaction_id: String,
    pub invoice_date: String, // YYYY-MM-DD
    pub total_amount: f64,
    pub description: String,
}

/// POST /invoices
/// Create a new invoice
pub async fn create_invoice(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<CreateInvoiceRequest>,
) -> Result<HttpResponse, AppError> {
    let service = InvoicesService::new(db.get_ref().clone());

    let req = ServiceCreateRequest {
        order_id: payload.order_id,
        transaction_id: payload.transaction_id.clone(),
        invoice_date: NaiveDate::parse_from_str(&payload.invoice_date, "%Y-%m-%d")
            .map_err(|_| AppError::BadRequest("Invalid date format, expected YYYY-MM-DD".into()))?,
        total_amount: payload.total_amount,
        description: payload.description.clone(),
    };

    let invoice = service.create_invoice(req).await?;
    Ok(HttpResponse::Ok().json(invoice))
}

/// GET /invoices
/// Fetch all invoices
pub async fn list_invoices(
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, AppError> {
    let service = InvoicesService::new(db.get_ref().clone());
    let invoices = service.get_all_invoices().await?;
    Ok(HttpResponse::Ok().json(invoices))
}

/// GET /invoices/{id}
/// Fetch a single invoice by ID
pub async fn get_invoice(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let service = InvoicesService::new(db.get_ref().clone());
    let invoice = service.get_all_invoices().await?; // or filter by ID if needed
    let invoice = invoice.into_iter().find(|i| i.invoice_id == id).ok_or(AppError::NotFound("Invoice not found.".into()))?;
    Ok(HttpResponse::Ok().json(invoice))
}

/// GET /invoices/order/{order_id}
/// Fetch invoices linked to a specific order
pub async fn get_invoice_by_order(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    let service = InvoicesService::new(db.get_ref().clone());
    let invoices = service.get_invoices_by_order(order_id).await?;
    Ok(HttpResponse::Ok().json(invoices))
}

/// DELETE /invoices/{id}
/// Delete an invoice
pub async fn delete_invoice(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let invoice_id = path.into_inner();
    let service = InvoicesService::new(db.get_ref().clone());
    service.delete_invoice(invoice_id).await?;
    Ok(HttpResponse::Ok().json("Invoice deleted successfully"))
}