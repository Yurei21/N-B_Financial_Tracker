use actix_web::{web, HttpResponse};
use crate::handlers::{
    users, orders, expenses, invoices, reports, dashboard, registration,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // 👤 User routes
            .route("/register", web::post().to(users::register))
            .route("/login", web::post().to(users::login))
            .route("/forgot-password", web::post().to(users::forgot_password))
            .route("/forgot-registration-code", web::post().to(registration::forgot_code))
            .route("/reset-registration-code", web::post().to(registration::reset_code))

            // 📦 Orders routes
            .route("/orders", web::post().to(orders::create_order))
            .route("/orders", web::get().to(orders::list_orders))
            .route("/orders/{id}", web::put().to(orders::update_order))
            .route("/orders/{id}", web::delete().to(orders::delete_order))
            .route("/orders/{id}", web::get().to(orders::get_order))

            // 💸 Expenses routes
            .route("/expenses", web::post().to(expenses::create_expense))
            .route("/expenses", web::get().to(expenses::list_expenses))
            .route("/expenses/{id}", web::put().to(expenses::update_expense))
            .route("/expenses/{id}", web::delete().to(expenses::delete_expense))
            .route("/expenses/{id}", web::get().to(expenses::get_expense))

            // 🧾 Invoices routes
            .route("/invoices", web::get().to(invoices::list_invoices))
            .route("/invoices/{id}", web::get().to(invoices::get_invoice))
            .route("/invoices/order/{order_id}", web::get().to(invoices::get_invoice_by_order))

            // 📊 Reports routes
            .route("/reports", web::post().to(reports::generate_report))
            .route("/reports", web::get().to(reports::list_reports))
            .route("/reports/{month}", web::get().to(reports::get_report_by_month))

            // 📈 Dashboard summary
            .route("/dashboard", web::get().to(dashboard::summary))
    );
}
