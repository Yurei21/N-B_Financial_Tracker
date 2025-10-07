use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;

use crate::{
    middleware::auth::AuthenticatedUser,
    services::reports,
    errors::AppError,
};

/// POST /reports/generate
/// Generate or refresh a report for a specific month (YYYY-MM)
pub async fn generate_report(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    body: web::Json<reports::GenerateReportRequest>,
) -> Result<HttpResponse, AppError> {
    let report = reports::generate_report(&db, user.user_id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(report))
}

/// GET /reports
/// List all generated reports
pub async fn list_reports(
    db: web::Data<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let list = reports::list_reports(&db).await?;
    Ok(HttpResponse::Ok().json(list))
}

/// GET /reports/{id}
/// Fetch a single report by ID
pub async fn get_report(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let report = reports::get_report(&db, id).await?;
    Ok(HttpResponse::Ok().json(report))
}

/// GET /reports/month/{month}
/// Fetch report for a specific month (YYYY-MM)
pub async fn get_report_by_month(
    db: web::Data<DatabaseConnection>,
    month: web::Path<String>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let month_str = month.into_inner();
    let report = reports::get_report_by_month(&db, &month_str).await?;
    Ok(HttpResponse::Ok().json(report))
}
