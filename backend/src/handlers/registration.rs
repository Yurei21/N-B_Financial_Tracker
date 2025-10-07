use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    services::registration::{RegistrationService, ForgotRegistrationRequest, ResetRegistrationRequest},
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
    pub new_registration_code: String,
}

/// POST /registration/forgot-code
pub async fn forgot_code(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<ForgotCodeRequest>,
) -> Result<HttpResponse, AppError> {
    let service = RegistrationService::new(db.get_ref().clone());

    let req = ForgotRegistrationRequest {
        email: payload.email.clone(),
    };

    let code = service.forgot_registration_code(req).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "verification_code": code })))
}

/// POST /registration/reset-code
pub async fn reset_code(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<ResetCodeRequest>,
) -> Result<HttpResponse, AppError> {
    let service = RegistrationService::new(db.get_ref().clone());

    let req = ResetRegistrationRequest {
        email: payload.email.clone(),
        verification_code: payload.verification_code.clone(),
        new_registration_code: payload.new_registration_code.clone(),
    };

    let message = service.reset_registration_code(req).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": message })))
}