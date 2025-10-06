use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use crate::config::Config;
use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
}

pub struct AuthenticatedUser {
    pub user_id: i32,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Clone the app data so we can use it inside async
        let config = req.app_data::<actix_web::web::Data<Config>>().cloned();

        Box::pin(async move {
            let config = match config {
                Some(c) => c,
                None => return Err(AppError::Unauthorized.into()),
            };

            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");

            // Expect header format: "Bearer <token>"
            if !auth_header.starts_with("Bearer ") {
                return Err(AppError::Unauthorized.into());
            }

            let token = auth_header.trim_start_matches("Bearer ").trim();

            let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());

            match decode::<Claims>(
                token,
                &decoding_key,
                &Validation::default(),
            ) {
                Ok(data) => Ok(AuthenticatedUser {
                    user_id: data.claims.user_id,
                }),
                Err(_) => Err(AppError::Unauthorized.into()),
            }
        })
    }
}
