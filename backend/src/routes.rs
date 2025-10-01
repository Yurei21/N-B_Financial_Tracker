use actix_web::web::{self, ServiceConfig};
use crate::handlers;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.
        service(
            web::scope("/api")
            // auth-free endpoints
                .service()
        )
}