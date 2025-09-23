use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create().if_not_exists().col(
                    ColumnDef::new(Users::UserId)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
                )
                .col(ColumnDef::new(Users::UserName).string_len(255))
                .col(ColumnDef::new(Users::PasswordHash).text().not_null())
                .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    UserId,
    UserName,
    PasswordHash,
}
