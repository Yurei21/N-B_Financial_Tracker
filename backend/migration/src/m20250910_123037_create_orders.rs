use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Orders::OrderId).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Orders::PatientName).string().not_null())
                    .col(ColumnDef::new(Orders::OrderDate).date().not_null())
                    .col(ColumnDef::new(Orders::TotalAmount).decimal(12,2).not_null())
                    .to_owned()
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
enum Orders {
    Table,
    OrderId,
    PatientName,
    OrderDate,
    TotalAmount,
}
