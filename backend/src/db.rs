use sea_orm::{Database, DatabaseConnection};
use crate::config::Config;

pub async fn connect(config: &Config) -> DatabaseConnection {
    Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to database.")
}