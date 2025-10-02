use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RegistrationCodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RegistrationCodes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RegistrationCodes::CodeHash).string().not_null())
                    .col(
                        ColumnDef::new(RegistrationCodes::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RegistrationCodes::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum RegistrationCodes {
    Table,
    Id,
    CodeHash,
    UpdatedAt,
}
