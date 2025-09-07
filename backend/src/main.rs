mod db;

use actix_web::get{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use sqlx::SqlitePool;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::get_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .route("/health", web::get().to(health))
            .service(hello)  
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}