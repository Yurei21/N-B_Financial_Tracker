use actix_web::{HttpResponse, http::StatusCode, ResponseError};
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found")]
    NotFound,

    #[error("Internal error")]
    InternalError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}