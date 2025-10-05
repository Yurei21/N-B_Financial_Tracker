use actix_web::{get, web, HttpResponse, Responder};

#[get("/reports/monthly")]
async fn generate_monthly_report() -> impl Responder {
    HttpResponse::Ok().body("Generate monthly report endpoint")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(generate_monthly_report);
}
