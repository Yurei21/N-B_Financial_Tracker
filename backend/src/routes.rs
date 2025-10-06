use actix_web::web;
use crate::handlers::{
    users, orders, expenses, invoices, reports, dashboard,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // 👤 User routes
            .service(users::register)
            .service(users::login)
            .service(users::forgot_password)
            .service(users::forgot_registration_code)

            // 📦 Orders routes
            .service(orders::create_order)
            .service(orders::list_orders)
            .service(orders::update_order)
            .service(orders::delete_order)

            // 💸 Expenses routes
            .service(expenses::create_expense)
            .service(expenses::list_expenses)
            .service(expenses::update_expense)
            .service(expenses::delete_expense)

            // 🧾 Invoices routes
            .service(invoices::list_invoices)
            .service(invoices::get_invoice_by_order)

            // 📊 Reports routes
            .service(reports::generate_report)
            .service(reports::list_reports)

            // 📈 Dashboard summary
            .service(dashboard::summary)
    );
}
