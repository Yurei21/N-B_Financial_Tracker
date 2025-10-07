use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    services::registration::{
        send_verification_email,
        verify_and_reset_registration_code,
    },
    errors::AppError,
};

#[derive(Debug, Deserialize)]
pub struct ForgotCodeRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetCodeRequest {
    pub email: String,
    pub verification_code: String,
}

// POST /registration/forgot-code
pub async fn forgot_code (
    db: web::Data<DatabaseConnection>,
    payload: web::Json<ForgotCodeRequest>,
) -> Result<HttpResponse, AppError> {
    send_verification_email(&db, &payload.email).await?;
    Ok(HttpResponse::Ok().json("Verification email sent"))
}

// POST /registration/reset-code
pub async fn reset_code (
    db: web::Data<DatabaseConnection>,
    payload: web::Json<ResetCodeRequest>,
) -> Result<HttpResponse, AppError> {
    verify_and_reset_registration_code(&db, &payload.email, &payload.verification_code).await?;
    Ok(HttpResponse::Ok().json("Registration code has been reset"))
}