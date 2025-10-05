use actix_web::{get, web, HttpResponse, Responder};

#[get("/dashboard/summary")]
async fn get_summary() -> impl Responder {
    HttpResponse::Ok().body("Dashboard summary endpoint")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_summary);
}