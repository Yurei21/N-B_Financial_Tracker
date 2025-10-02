use actix_web::web::{self, ServiceConfig};
use crate::handlers;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg
        .service(
            web::scope("/api")
                // ---- Health check ----
                .service(web::resource("/health").route(web::get().to(handlers::health)))

                // ---- Auth-free endpoints ----
                .service(web::resource("/register").route(web::post().to(handlers::register)))
                .service(web::resource("/login").route(web::post().to(handlers::login)))
                .service(web::resource("/forgot-password").route(web::post().to(handlers::forgot_password)))
                .service(web::resource("/forgot-registration-code").route(web::post().to(handlers::forgot_registration_code)))
                .service(web::resource("/reset-registration-code").route(web::post().to(handlers::reset_registration_code)))

                // ---- Protected endpoints (JWT) ----
                .service(
                    web::scope("/orders")
                        .route("", web::post().to(handlers::create_order))
                        .route("", web::get().to(handlers::list_orders))
                        .route("/{id}", web::get().to(handlers::get_order))
                        .route("/{id}", web::put().to(handlers::update_order))
                        .route("/{id}", web::delete().to(handlers::delete_order))
                )
                .service(
                    web::scope("/expenses")
                        .route("", web::post().to(handlers::create_expense))
                        .route("", web::get().to(handlers::list_expenses))
                        .route("/{id}", web::get().to(handlers::get_expense))
                        .route("/{id}", web::put().to(handlers::update_expense))
                        .route("/{id}", web::delete().to(handlers::delete_expense))
                )
                .service(
                    web::scope("/invoices")
                        .route("", web::get().to(handlers::list_invoices))
                        .route("/{id}", web::get().to(handlers::get_invoice))
                )
                .service(
                    web::scope("/reports")
                        .route("", web::get().to(handlers::list_reports))
                        .route("/monthly", web::post().to(handlers::generate_monthly_report))
                )
                .service(
                    web::scope("/dashboard")
                        .route("/summary", web::get().to(handlers::dashboard_summary))
                )
        );
}
