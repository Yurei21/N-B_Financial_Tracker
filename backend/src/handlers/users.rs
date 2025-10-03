use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/register")]
async fn register() -> impl Responder {
    HttpResponse::Ok().body("User register endpoint")
}

#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("User login endpoint")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register).service(login);
}