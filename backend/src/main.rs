use actix_web::{App, HttpServer, web, HttpResponse};
use backend::config::Config;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    println!("🚀 Starting server at http://{}:{}", config.server_host, config.server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone())) 
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Financial Tracker API Running ✅") }))
    })
    .bind((config.server_host.clone(), config.server_port))?
    .run()
    .await
}