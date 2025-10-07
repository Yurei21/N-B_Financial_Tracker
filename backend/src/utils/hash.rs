use argon2::{self, Config as ArgonConfig};
use crate::errors::AppError;

/// Hash any sensitive value (like passwords or codes)
pub fn hash_value(value: &str) -> Result<String, AppError> {
    let salt = b"financial_tracker_salt"; 
    argon2::hash_encoded(value.as_bytes(), salt, &ArgonConfig::default())
        .map_err(|_| AppError::InternalServerError)
}

/// Verify a plaintext value against its Argon2 hash
pub fn verify_hash(value: &str, hash: &str) -> Result<bool, AppError> {
    argon2::verify_encoded(hash, value.as_bytes())
        .map_err(|_| AppError::InternalServerError)
}
