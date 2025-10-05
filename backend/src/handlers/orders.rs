use actix_web::{get, post, put, delete, web, HttpResponse, Responder};

#[post("/orders")]
async fn create_order() -> impl Responder {
    HttpResponse::Ok().body("Create Order endpoint")
}

#[get("/orders")]
async fn list_orders() -> impl Responder {
    HttpResponse::Ok().body("List Order endpoint")
}

#[put("/orders/{id}")]
async fn update_order(path: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Update Order {}", path.into_inner()))
}

#[delete("/orders/{id}")]
async fn delete_order(path: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Delete Order {}", path.into_inner()))
}

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(create_order)
        .service(list_orders)
        .service(update_order)
        .service(delete_order);
}