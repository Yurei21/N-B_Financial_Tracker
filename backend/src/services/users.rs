use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use rand::{Rng, thread_rng};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, PasswordHash}};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};

use crate::Config as ArgonConfig;
use crate::{
    entities::{users, registration_codes},
    errors::AppError,
    config::Config,
    middleware::auth::Claims,
};

#[derive(Clone)]
pub struct UserService {
    pub db: DatabaseConnection,
    pub jwt_secret: String,
}

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

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub username: String,
    pub registration_code: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: i32,
    pub username: String,
}

impl UserService {
    pub fn new(db: DatabaseConnection, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }

    pub async fn register_user(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        // Check registration code validity
        let reg_code = registration_codes::Entity::find()
            .one(&self.db)
            .await?
            .ok_or(AppError::BadRequest("Invalid registration code".into()))?;

        if reg_code.code_hash != req.registration_code {
            return Err(AppError::BadRequest("Invalid registration code".into()));
        }

        // Check if username exists
        let existing = users::Entity::find()
            .filter(users::Column::Username.eq(req.username.clone()))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::BadRequest("Username already exists".into()));
        }

        // Generate salt
        let mut rng = thread_rng();
        let salt_bytes: [u8; 16] = rng.r#gen();
        let salt = SaltString::b64_encode(&salt_bytes).map_err(|_| AppError::InternalError)?;

        // Hash password
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|_| AppError::InternalError)?
            .to_string();

        let new_user = users::ActiveModel {
            username: Set(req.username.clone()),
            password_hash: Set(hash),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        // Generate JWT
        let token = self.generate_jwt(new_user.user_id)?;

        Ok(AuthResponse {
            token,
            user_id: new_user.user_id,
            username: new_user.username,
        })
    }

    pub async fn login_user(&self, req: LoginRequest) -> Result<AuthResponse, AppError> {
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(req.username.clone()))
            .one(&self.db)
            .await?
            .ok_or(AppError::BadRequest("Invalid username or password".into()))?;

        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| AppError::InternalError)?;
        let valid = argon2.verify_password(req.password.as_bytes(), &parsed_hash).is_ok();

        if !valid {
            return Err(AppError::BadRequest("Invalid username or password".into()));
        }

        let token = self.generate_jwt(user.user_id)?;

        Ok(AuthResponse {
            token,
            user_id: user.user_id,
            username: user.username,
        })
    }

    pub async fn forgot_password(&self, req: ForgotPasswordRequest) -> Result<(), AppError> {
        // Find user by username
        let mut user: users::ActiveModel = users::Entity::find()
            .filter(users::Column::Username.eq(req.username.clone()))
            .one(&self.db)
            .await?
            .ok_or(AppError::BadRequest("User not found".into()))?
            .into();

        // Validate registration code
        let reg_code = registration_codes::Entity::find()
            .one(&self.db)
            .await?
            .ok_or(AppError::BadRequest("Invalid registration code".into()))?;

        if reg_code.code_hash != req.registration_code {
            return Err(AppError::BadRequest("Invalid registration code".into()));
        }

        // Hash new password
        let mut rng = thread_rng();
        let salt_bytes: [u8; 16] = rng.r#gen();
        let salt = SaltString::b64_encode(&salt_bytes).map_err(|_| AppError::InternalError)?;
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(req.new_password.as_bytes(), &salt)
            .map_err(|_| AppError::InternalError)?
            .to_string();

        // Update user password
        user.password_hash = Set(hash);
        user.update(&self.db).await?;

        Ok(())
    }

    fn generate_jwt(&self, user_id: i32) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .ok_or(AppError::InternalError)?
            .timestamp() as usize;

        let claims = Claims { user_id, exp: expiration };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::InternalError)
    }
}