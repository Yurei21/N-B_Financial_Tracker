use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Expenses::ExpenseId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Expenses::Description).string().not_null())
                    .col(ColumnDef::new(Expenses::Label).string().not_null())
                    .col(ColumnDef::new(Expenses::Amount).decimal_len(12, 2).not_null())
                    .col(ColumnDef::new(Expenses::ExpenseDate).date().not_null())
                    .col(ColumnDef::new(Expenses::CreatedBy).integer().null())
                    .col(ColumnDef::new(Expenses::ModifiedBy).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-expenses-created_by")
                            .from(Expenses::Table, Expenses::CreatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-expenses-modified_by")
                            .from(Expenses::Table, Expenses::ModifiedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Expenses::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum Expenses {
    Table,
    ExpenseId,
    Description,
    Label,
    Amount,
    ExpenseDate,
    CreatedBy,
    ModifiedBy,
}

#[derive(Iden)]
enum Users {
    Table,
    UserId,
}
