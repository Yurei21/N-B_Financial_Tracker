pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250910_123032_create_users;
mod m20250910_123037_create_orders;
mod m20250910_123043_create_invoices;
mod m20250910_123048_create_expenses;
mod m20250910_123054_create_reports;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250910_123032_create_users::Migration),
            Box::new(m20250910_123037_create_orders::Migration),
            Box::new(m20250910_123043_create_invoices::Migration),
            Box::new(m20250910_123048_create_expenses::Migration),
            Box::new(m20250910_123054_create_reports::Migration),
        ]
    }
}
