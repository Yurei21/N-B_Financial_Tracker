use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub registration_enabled: bool,
    pub registration_secret: String,
}