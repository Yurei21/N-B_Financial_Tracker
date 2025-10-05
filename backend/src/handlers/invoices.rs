use actix_web::{get, web, HttpResponse, Responder};

#[get("/invoices")]
async fn list_invoices() -> impl Responder {
    HttpResponse::Ok().body("List invoices endpoint")
}

#[get("/invoices/{id}")]
async fn get_invoice(path: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Get invoice {}", path.into_inner()))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_invoices)
       .service(get_invoice);
}