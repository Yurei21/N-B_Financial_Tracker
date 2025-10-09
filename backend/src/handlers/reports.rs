use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use serde::Deserialize;
use sea_orm::DatabaseConnection;

use crate::{
    middleware::auth::AuthenticatedUser,
    services::reports::{ReportsService, MonthlyReport as ServiceMonthlyReport},
    errors::AppError,
};

#[derive(Debug, Deserialize)]
pub struct GenerateReportRequest {
    pub month: String, // YYYY-MM
}

/// POST /reports
/// Generate a monthly report
pub async fn generate_report(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<GenerateReportRequest>,
) -> Result<HttpResponse, AppError> {
    let service = ReportsService::new(db.get_ref().clone());

    // Parse YYYY-MM into NaiveDate (first day of month)
    let month = NaiveDate::parse_from_str(&(payload.month.clone() + "-01"), "%Y-%m-%d")
    .map_err(|_| AppError::BadRequest("Invalid month format, expected YYYY-MM".into()))?;

    let report = service.generate_monthly_report(month).await?;
    Ok(HttpResponse::Ok().json(report))
}

/// GET /reports
/// Fetch all generated reports
pub async fn list_reports(
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, AppError> {
    let service = ReportsService::new(db.get_ref().clone());
    let reports = service.get_all_reports().await?;
    Ok(HttpResponse::Ok().json(reports))
}

/// GET /reports/{month}
/// Fetch report for a specific month (YYYY-MM)
pub async fn get_report_by_month(
    db: web::Data<DatabaseConnection>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let month_str = path.into_inner();
    let service = ReportsService::new(db.get_ref().clone());

    // Parse YYYY-MM into NaiveDate (first day of month)
    let month = NaiveDate::parse_from_str(&(month_str + "-01"), "%Y-%m-%d")?;

    let report = service.get_report_by_month(month).await?;
    Ok(HttpResponse::Ok().json(report))
}