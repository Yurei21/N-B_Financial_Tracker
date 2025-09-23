use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // users
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::UserId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).string().not_null())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .to_owned(),
            )
            .await?;

        // orders
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
                    .to_owned(),
            )
            .await?;

        // expenses
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
                    .to_owned(),
            )
            .await?;

        // invoices
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
                            .name("fk-invoice-order")
                            .from(Invoices::Table, Invoices::OrderId)
                            .to(Orders::Table, Orders::OrderId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // reports
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
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Reports::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Invoices::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Expenses::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Orders::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    UserId,
    Username,
    PasswordHash,
}

#[derive(Iden)]
enum Orders {
    Table,
    OrderId,
    PatientName,
    OrderDate,
    TotalAmount,
    Description,
}

#[derive(Iden)]
enum Expenses {
    Table,
    ExpenseId,
    Description,
    Label,
    Amount,
    ExpenseDate,
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
