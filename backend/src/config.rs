use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub registration_enabled: bool,
    pub registration_secret: String,
}

impl Config {
    pub fn from_env() -> self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env");
        
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a number");
        
        let registration_enabled = env::var("REGISTRATION_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        let registration_secret = env::var("REGISTRATION_SECRET")
            .unwrap_or_else(|_| "".to_string());
        
        Self{
            database_url,
            host,
            registration_enabled,
            registration_secret,
        }
    }
}