use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use argon2::{self, Config as ArgonConfig};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};

use crate::{
    entities::{users, registration_code},
    errors::AppError,
    config::Config,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub registration_code: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}

pub async fn register_user(
    db: &DatabaseConnection,
    config: &Config,
    req: RegisterRequest
) -> Result<AuthResponse, AppError>{
    //Check registration code validity
    let reg_code = registration_code::Entity::find()
        .filter(registration_code)
        .one(db)
        .await?
        .ok_or(AppError::BadRequest("Invalid Registration code".into()))?;
    
    //Check if username exists
    let existing = users::Entity::find()
        .filter(users::Column::Username.eq(req.username.clone()))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Username already exists".into()));
    }

    //Hash Password
    let salt = rand::random::<[u8, 16]>();
    let hash = argon2::hash_encoded(req.password.as_bytes(), &salt, &ArgonConfig::default())
        .map_err(|_| AppError::Internal("Failed to hash the password".into()))?;

    let new_user = users::ActiveModel {
        username: Set(req.username.clone()),
        password_hash: Set(hash.clone),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let token = generate_jwt(config, new_user.user_id)?;

    Ok(AuthResponse {
        token,
        user_id,
        username,
    })
}

pub async fn login_user(
    db: &DatabaseConnection,
    config: &Config,
    req: LoginRequest,
) -> Result<AuthResponse, AppError>{
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(req.username.clone()))
        .one(db)
        .await?
        .ok_or(AppError::BadRequest("Invalid username or Password".into()))?;

    let valid = argon2::verify_encoded(&user.password_hash, req.password.as_bytes())
        .map_err(|_| AppError::Internal("Password verification failed".into()))?;
    
    if !valid {
        return Err(AppError::BadRequest("Invalid username or Password".into()));
    }

    let token = generate_jwt(config, user.user_id)?;

    Ok(AuthResponse {
        token,
        user_id: user.user_id,
        username: user.username
    })
}

fn generate_jwt (config: &Config, user_id: i32) -> Result<String, AppError> {
    use crate::middleware::auth::Claims

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { user_id, exp: expiration };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal("Failed to encode JWT".into()))?;

    Ok(token)
}