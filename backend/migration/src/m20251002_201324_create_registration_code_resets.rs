use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RegistrationCodeResets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RegistrationCodeResets::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RegistrationCodeResets::Email)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RegistrationCodeResets::HashedVerificationCode)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RegistrationCodeResets::ExpiresAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RegistrationCodeResets::Used)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RegistrationCodeResets::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum RegistrationCodeResets {
    Table,
    Id,
    Email,
    HashedVerificationCode,
    ExpiresAt,
    Used,
}
