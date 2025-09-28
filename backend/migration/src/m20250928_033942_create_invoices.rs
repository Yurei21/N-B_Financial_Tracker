use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Invoices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Invoices::InvoiceId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Invoices::OrderId).integer().not_null())
                    .col(ColumnDef::new(Invoices::TransactionId).string().not_null())
                    .col(ColumnDef::new(Invoices::InvoiceDate).date().not_null())
                    .col(ColumnDef::new(Invoices::TotalAmount).decimal_len(12, 2).not_null())
                    .col(ColumnDef::new(Invoices::Description).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-invoices-order")
                            .from(Invoices::Table, Invoices::OrderId)
                            .to(Orders::Table, Orders::OrderId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Invoices::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum Invoices {
    Table,
    InvoiceId,
    OrderId,
    TransactionId,
    InvoiceDate,
    TotalAmount,
    Description,
}

#[derive(Iden)]
enum Orders {
    Table,
    OrderId,
}