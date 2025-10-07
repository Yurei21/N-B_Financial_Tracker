use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use crate::{
    services::users::{register_user, login_user},
    entities::users,
    errors::AppError,
};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub registration_code: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// POST /register
pub async fn register(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    register_user(&db, &payload.username, &payload.password, &payload.registration_code).await?;
    Ok(HttpResponse::Created().json("User registered successfully"))
}

/// POST /login
pub async fn login(
    db: web::Data<DatabaseConnection>,
    config: web::Data<crate::config::Config>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let token = login_user(&db, &config, &payload.username, &payload.password).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}

/// (Optional) GET /me — returns current authenticated user
use crate::middleware::auth::AuthenticatedUser;

pub async fn get_me(
    user: AuthenticatedUser,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, AppError> {
    use sea_orm::EntityTrait;

    let user_data = users::Entity::find_by_id(user.user_id)
        .one(db.get_ref())
        .await?
        .ok_or(AppError::NotFound("User not found".into()))?;

    Ok(HttpResponse::Ok().json(user_data))
}
