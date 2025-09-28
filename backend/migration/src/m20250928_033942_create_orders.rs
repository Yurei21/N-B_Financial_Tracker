use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Orders::OrderId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Orders::PatientName).string().not_null())
                    .col(ColumnDef::new(Orders::OrderDate).date().not_null())
                    .col(ColumnDef::new(Orders::TotalAmount).decimal_len(12, 2).not_null())
                    .col(ColumnDef::new(Orders::Description).string().not_null())
                    .col(ColumnDef::new(Orders::CreatedBy).integer().null())
                    .col(ColumnDef::new(Orders::ModifiedBy).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-orders-created_by")
                            .from(Orders::Table, Orders::CreatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-orders-modified_by")
                            .from(Orders::Table, Orders::ModifiedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Orders::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum Orders {
    Table,
    OrderId,
    PatientName,
    OrderDate,
    TotalAmount,
    Description,
    CreatedBy,
    ModifiedBy,
}

#[derive(Iden)]
enum Users {
    Table,
    UserId,
}