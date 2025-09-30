use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, http::header};
use futures::future::{ok, err, Ready};
use serder::{Deserialize};
use jsonwebtoken::{decode, Validation, DecodingKey, Algorithm};
use crate::config::Config;
use actix_web::web::Data;
use crate::errors::AppError;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: i64,
    exp: usize,
}

pub struct AuthUser(pub i32);

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type config = ();
    
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let cfg = req.app_data::<Data<Config>>().cloned();

        let header = match req.headers().get(header::AUTHORIZATION){
            Some(h) => h.to_str().unwrap_or("").to_string(),
            None => return err(AppError::Unauthorized.into()),
        };

        if !header.to_lowercase().starts_with("bearer ") {
            return err(AppError::Unauthorized.into());
        }
        let token = header[7..].trim();

        let config = match cfg {
            Some(c) => c,
            None => return err(AppError::InternalError.into()),
        };

        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_ref());
        match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)){
            Ok(token_data) => {
                let uid = token_data.claims.sub as i32;
                ok(AuthUser(uid));
            }
            Err(_) => err(AppError::Unauthorized.into()),
        }
    }
}