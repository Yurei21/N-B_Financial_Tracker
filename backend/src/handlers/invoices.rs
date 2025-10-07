use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;

use crate::{
    middleware::auth::AuthenticatedUser,
    services::invoices,
    errors::AppError,
};

/// GET /invoices
/// Fetch all invoices (for the logged-in user)
pub async fn list_invoices(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let result = invoices::list_invoices(&db, user.user_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /invoices/{id}
/// Fetch a single invoice by its ID
pub async fn get_invoice(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let invoice = invoices::get_invoice(&db, id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(invoice))
}

/// GET /invoices/order/{order_id}
/// Fetch the invoice linked to a specific order
pub async fn get_invoice_by_order(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    let invoice = invoices::get_invoice_by_order(&db, order_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(invoice))
}
