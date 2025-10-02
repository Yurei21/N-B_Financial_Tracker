pub use sea_orm_migration::prelude::*;

mod m20250928_033942_create_users;
mod m20250928_033942_create_orders;
mod m20250928_033942_create_expenses;
mod m20250928_033942_create_invoices;
mod m20250928_033942_create_reports;
mod m20251002_093643_create_registration_codes;
mod m20251002_201324_create_registration_code_resets;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250928_033942_create_users::Migration),
            Box::new(m20250928_033942_create_orders::Migration),
            Box::new(m20250928_033942_create_expenses::Migration),
            Box::new(m20250928_033942_create_invoices::Migration),
            Box::new(m20250928_033942_create_reports::Migration),
            Box::new(m20251002_093643_create_registration_codes::Migration),
            Box::new(m20251002_201324_create_registration_code_resets::Migration),
        ]
    }
}
