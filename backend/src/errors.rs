use actix_web::{HttpResponse, http::StatusCode, ResponseError};
use thiserror::Error;
use serde::Serialize;
use sea_orm::DbErr;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(#[from] DbErr),

    #[error("Unauthorized")]
    Unauthorized,

    //#[error("Date parse error: {0}")]
   // DataParseError(#[from] chrono::ParseError)

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found")]
    NotFound(String),

    #[error("Internal error")]
    InternalError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = ErrorResponse {
            error: self.to_string(),
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}