use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::services::users as user_service;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct AuthPayload {
    pub username 
}