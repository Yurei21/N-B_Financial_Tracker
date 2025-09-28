use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reports::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reports::ReportId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reports::Month).date().not_null())
                    .col(ColumnDef::new(Reports::TotalOrders).integer().not_null())
                    .col(ColumnDef::new(Reports::TotalIncome).decimal_len(12, 2).not_null())
                    .col(ColumnDef::new(Reports::TotalExpenses).decimal_len(12, 2).not_null())
                    .col(ColumnDef::new(Reports::NetProfit).decimal_len(12, 2).not_null())
                    .col(
                        ColumnDef::new(Reports::GeneratedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Reports::DailyData).json_binary().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Reports::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum Reports {
    Table,
    ReportId,
    Month,
    TotalOrders,
    TotalIncome,
    TotalExpenses,
    NetProfit,
    GeneratedAt,
    DailyData,
}