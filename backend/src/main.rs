use actix_web::get{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message" : "Hello From Actix"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(hello)  
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}