use actix_web::web;
use crate::handlers;

use crate::handlers::{
    users,
    orders,
    expenses,
    invoices,
    dashboard,
    reports,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.
        service(
            web::scope("/api")
            // auth-free endpoints
                .service()
        )
}