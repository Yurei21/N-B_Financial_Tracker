mod db;
mod config;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use config::Config;
use db::connect;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let db = connect(&config).await;
    println!("Connected to the database");
    Ok(())
}