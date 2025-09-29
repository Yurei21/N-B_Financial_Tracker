use actix_web::{App, HttpServer, web, HttpResponse};
use backend::config::Config;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let config_data = actix_web::web::Data::new(config.clone());
    let db = db::connect(&config).await;
    let db_data = actix_web::web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(config_data.clone())
            .app_data(db_data.clone())
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Financial Tracker API Running ✅") }))
    })
    .bind((config.server_host.clone(), config.server_port))?
    .run()
    .await
}