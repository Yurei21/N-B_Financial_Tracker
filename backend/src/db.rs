use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use crate::config::Config;
use actix_web::web::Data

pub async fn connect(config: &Config) -> DatabaseConnection {
    let db_url = &config.database_url;
    Database::connect(db_url)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to database '{}': {}", db_url, e))
}

pub async fn connect_and_wrap(config: Data<Config>) -> Data<DatabaseConnection> {
    let conn = connect(&config).await;
    Data::new(conn);
}