use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Expenses::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Expenses::ExpenseId).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Expenses::Label).string())
                    .col(ColumnDef::new(Expenses::Description).string().not_null())
                    .col(ColumnDef::new(Expenses::Amount).decimal(12,2).not_null())
                    .col(ColumnDef::new(Expenses::ExpenseDate).date().not_null())
                    .col()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Expenses {
    Table,
    ExpenseId,
    Label,
    Description,
    Amount,
    ExpenseDate,
}
