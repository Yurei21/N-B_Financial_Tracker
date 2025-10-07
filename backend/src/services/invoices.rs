use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::{
    entities::{invoices, orders},
    errors::AppError,
};

#[derive(Clone)]
pub struct InvoicesService {
    pub db: DatabaseConnection,
}

#[derive(Deserialize)]
pub struct CreateInvoiceRequest {
    pub order_id: i32,
    pub transaction_id: String,
    pub invoice_date: NaiveDate,
    pub total_amount: f64,
    pub description: String,
}

impl InvoicesService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Generate an invoice for an order
    pub async fn create_invoice(&self, req: CreateInvoiceRequest) -> Result<invoices::Model, AppError> {
        // Verify the order exists
        let order = orders::Entity::find_by_id(req.order_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::BadRequest("Order not found".into()))?;

        let new_invoice = invoices::ActiveModel {
            order_id: Set(order.order_id),
            transaction_id: Set(req.transaction_id),
            invoice_date: Set(req.invoice_date),
            total_amount: Set(req.total_amount),
            description: Set(req.description),
            ..Default::default()
        };

        let invoice = new_invoice.insert(&self.db).await?;
        Ok(invoice)
    }

    /// Fetch all invoices
    pub async fn get_all_invoices(&self) -> Result<Vec<invoices::Model>, AppError> {
        let all_invoices = invoices::Entity::find()
            .all(&self.db)
            .await?;
        Ok(all_invoices)
    }

    /// Fetch invoices by order ID
    pub async fn get_invoices_by_order(&self, order_id: i32) -> Result<Vec<invoices::Model>, AppError> {
        let invoices_list = invoices::Entity::find()
            .filter(invoices::Column::OrderId.eq(order_id))
            .all(&self.db)
            .await?;
        Ok(invoices_list)
    }

    /// Delete invoice
    pub async fn delete_invoice(&self, invoice_id: i32) -> Result<(), AppError> {
        let invoice: invoices::ActiveModel = invoices::Entity::find_by_id(invoice_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?
            .into();

        invoice.delete(&self.db).await?;
        Ok(())
    }
}
