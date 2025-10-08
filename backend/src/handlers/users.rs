use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use crate::{
    services::users::{UserService, RegisterRequest as ServiceRegisterRequest, LoginRequest as ServiceLoginRequest, ForgotPasswordRequest, UserResponse},
    entities::users,
    errors::AppError,
};
use crate::middleware::auth::AuthenticatedUser;
use crate::config::Config;

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

#[derive(Deserialize)]
pub struct ForgotPasswordPayload {
    pub username: String,
    pub registration_code: String,
    pub new_password: String,
}

/// POST /register
pub async fn register(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let config = Config::from_env().map_err(|_| AppError::InternalError)?; // Load env
    let service = UserService::new(db.get_ref().clone(), config.jwt_secret.clone());

    // Create a ServiceRegisterRequest struct
    let req = ServiceRegisterRequest {
        username: payload.username.clone(),
        password: payload.password.clone(),
        registration_code: payload.registration_code.clone(),
    };

    let auth_response = service.register_user(req).await?;

    Ok(HttpResponse::Created().json(auth_response))
}

/// POST /login
pub async fn login(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let config = Config::from_env().map_err(|_| AppError::InternalError)?;
    let service = UserService::new(db.get_ref().clone(), config.jwt_secret.clone());

    let req = ServiceLoginRequest {
        username: payload.username.clone(),
        password: payload.password.clone(),
    };

    let auth_response = service.login_user(req).await?;

    Ok(HttpResponse::Ok().json(auth_response))
}

/// GET /me — returns current authenticated user
pub async fn get_me(
    user: AuthenticatedUser,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, AppError> {
    use sea_orm::EntityTrait;

    let user_data = users::Entity::find_by_id(user.user_id)
        .one(db.get_ref())
        .await?
        .ok_or(AppError::NotFound("User not found".into()))?;

    let response = UserResponse {
        user_id: user_data.user_id,
        username: user_data.username,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// POST /forgot-password
pub async fn forgot_password(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<ForgotPasswordPayload>,
) -> Result<HttpResponse, AppError> {
    let config = Config::from_env().map_err(|_| AppError::InternalError)?;
    let service = UserService::new(db.get_ref().clone(), config.jwt_secret.clone());

    let req = ForgotPasswordRequest {
        username: payload.username.clone(),
        registration_code: payload.registration_code.clone(),
        new_password: payload.new_password.clone(),
    };

    service.forgot_password(req).await?;

    Ok(HttpResponse::Ok().json("Password reset successfully"))
}
