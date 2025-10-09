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

#[derive(Serialize)]
pub struct InvoiceResponse {
    pub invoice_id: i32,
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
    pub async fn create_invoice(&self, req: CreateInvoiceRequest) -> Result<InvoiceResponse, AppError> {
        // Verify the order exists
        let order = orders::Entity::find_by_id(req.order_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::BadRequest("Order not found".into()))?;

        // Check if an invoice already exists for this order
        if let Some(_) = invoices::Entity::find()
            .filter(invoices::Column::OrderId.eq(req.order_id))
            .one(&self.db)
            .await?
        {
            return Err(AppError::BadRequest("Invoice already exists for this order".into()));
        }

        // Insert the invoice
        let new_invoice = invoices::ActiveModel {
            order_id: Set(order.order_id),
            transaction_id: Set(req.transaction_id),
            invoice_date: Set(req.invoice_date),
            total_amount: Set(req.total_amount),
            description: Set(req.description.clone()),
            ..Default::default()
        };

        let invoice = new_invoice.insert(&self.db).await?;

        Ok(InvoiceResponse {
            invoice_id: invoice.invoice_id,
            order_id: invoice.order_id,
            transaction_id: invoice.transaction_id,
            invoice_date: invoice.invoice_date,
            total_amount: invoice.total_amount,
            description: invoice.description,
        })
    }

    /// Fetch all invoices
    pub async fn get_all_invoices(&self) -> Result<Vec<InvoiceResponse>, AppError> {
        let invoices_list = invoices::Entity::find().all(&self.db).await?;

        Ok(invoices_list
            .into_iter()
            .map(|inv| InvoiceResponse {
                invoice_id: inv.invoice_id,
                order_id: inv.order_id,
                transaction_id: inv.transaction_id,
                invoice_date: inv.invoice_date,
                total_amount: inv.total_amount,
                description: inv.description,
            })
            .collect())
    }

    /// Fetch invoices by order ID
    pub async fn get_invoices_by_order(&self, order_id: i32) -> Result<Vec<InvoiceResponse>, AppError> {
        let invoices_list = invoices::Entity::find()
            .filter(invoices::Column::OrderId.eq(order_id))
            .all(&self.db)
            .await?;

        Ok(invoices_list
            .into_iter()
            .map(|inv| InvoiceResponse {
                invoice_id: inv.invoice_id,
                order_id: inv.order_id,
                transaction_id: inv.transaction_id,
                invoice_date: inv.invoice_date,
                total_amount: inv.total_amount,
                description: inv.description,
            })
            .collect())
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
