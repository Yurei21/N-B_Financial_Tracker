use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set in .env");

    Database::connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Failed to connect to database: {}", db_url))
}