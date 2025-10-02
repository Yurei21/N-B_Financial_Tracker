use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub database_url_pg: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            database_url_pg: env::var("DATABASE_URL_PG").expect("DATABASE_URL_PG not set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not set"),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT must be a number"),
        }
    }
}
