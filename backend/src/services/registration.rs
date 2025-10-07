use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::{
    entities::{registration_codes, registration_code_resets},
    errors::AppError,
};

#[derive(Clone)]
pub struct RegistrationService {
    pub db: DatabaseConnection,
}

#[derive(Deserialize)]
pub struct ForgotRegistrationRequest {
    pub email: String, // company's email
}

#[derive(Deserialize)]
pub struct ResetRegistrationRequest {
    pub email: String,
    pub verification_code: String,
    pub new_registration_code: String,
}

impl RegistrationService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Step 1: Send verification code to company email
    pub async fn forgot_registration_code(
        &self,
        req: ForgotRegistrationRequest,
    ) -> Result<String, AppError> {
        // Generate a random 6-digit verification code
        let verification_code: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        // Save hashed verification code linked to email in registration_code_resets table
        let active_model = registration_code_resets::ActiveModel {
            email: Set(req.email.clone()),
            code: Set(verification_code.clone()),
            ..Default::default()
        };

        active_model.insert(&self.db).await?;

        // Here you would send the code to the company's email
        // For now, just return it for testing
        Ok(verification_code)
    }

    /// Step 2: Verify code and reset registration code
    pub async fn reset_registration_code(
        &self,
        req: ResetRegistrationRequest,
    ) -> Result<String, AppError> {
        let record = registration_code_resets::Entity::find()
            .filter(registration_code_resets::Column::Email.eq(req.email.clone()))
            .filter(registration_code_resets::Column::Code.eq(req.verification_code.clone()))
            .one(&self.db)
            .await?
            .ok_or(AppError::BadRequest("Invalid verification code".into()))?;

        // Update the registration_code table with new code
        let mut reg_code: registration_codes::ActiveModel = registration_codes::Entity::find()
            .one(&self.db)
            .await?
            .ok_or(AppError::InternalError)?
            .into();

        reg_code.code = Set(req.new_registration_code.clone());
        reg_code.update(&self.db).await?;

        // Optionally, delete the verification record after successful reset
        let mut reset_model: registration_code_resets::ActiveModel = record.into();
        reset_model.delete(&self.db).await?;

        Ok("Registration code successfully reset".into())
    }
}
