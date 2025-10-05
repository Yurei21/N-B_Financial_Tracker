use actix_web::{get, post, put, delete, web, HttpResponse, Responder};

#[post("/expenses")]
async fn create_expense() -> impl Responder {
    HttpResponse::Ok().body("Create expense endpoint")
}

#[get("/expenses")]
async fn list_expenses() -> impl Responder {
    HttpResponse::Ok().body("List expenses endpoint")
}

#[put("/expenses/{id}")]
async fn update_expense(path: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Update expense {}", path.into_inner()))
}

#[delete("/expenses/{id}")]
async fn delete_expense(path: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Delete expense {}", path.into_inner()))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_expense)
       .service(list_expenses)
       .service(update_expense)
       .service(delete_expense);
}