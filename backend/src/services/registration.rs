use sea_orm::{EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter};
use chrono::{Utc, Duration};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use argon2::{self, Config as ArgonConfig};
use crate::entities::{registration_code, registration_code_resets};
use crate::errors::AppError;
use crate::utils::hash::verify_hash
use sea_orm::DatabaseConnection;

fn generate_verification_code() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

pub async fn reset_registration_code (
    db: &DatabaseConnection,
    email: &str
)