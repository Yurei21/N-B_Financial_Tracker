use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter, ActiveValue};
use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};
use chrono::{Datelike, NaiveDate};
use crate::{
    entities::{orders, invoices, users, reports},
    errors::AppError,
    services::invoices::{InvoicesService, CreateInvoiceRequest},
    services::reports::ReportsService,
};

#[derive(Clone)]
pub struct OrdersService {
    pub db: DatabaseConnection,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub patient_name: String,
    pub order_date: NaiveDate,
    pub total_amount: f64,
    pub description: String,
    pub created_by: i32, // user_id
}

#[derive(Deserialize)]
pub struct UpdateOrderRequest {
    pub patient_name: Option<String>,
    pub order_date: Option<NaiveDate>,
    pub total_amount: Option<f64>,
    pub description: Option<String>,
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

#[derive(Serialize)]
pub struct CreateOrderResponse {
    pub order_id: i32,
    pub patient_name: String,
    pub order_date: NaiveDate,
    pub total_amount: f64,
    pub description: String,
    pub created_by: Option<i32>,
}

#[derive(Serialize)]
pub struct AllOrderResponse {
    pub order_id: i32,
    pub patient_name: String,
    pub order_date: NaiveDate,
    pub total_amount: f64,
    pub description: String,
    pub created_by: Option<i32>,
    pub modified_by: Option<i32>,
}

#[derive(Serialize)]
pub struct GetOrderResponse {
    pub order_id: i32,
    pub patient_name: String,
    pub order_date: NaiveDate,
    pub total_amount: f64,
    pub description: String,
    pub created_by: Option<i32>,
    pub modified_by: Option<i32>,
}

#[derive(Serialize)]
pub struct UpdateOrderResponse {
    pub order_id: i32,
    pub patient_name: String,
    pub order_date: NaiveDate,
    pub total_amount: f64,
    pub description: String,
    pub created_by: Option<i32>,
    pub modified_by: Option<i32>,
}

impl OrdersService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_order(&self, req: CreateOrderRequest) -> Result<(CreateOrderResponse, InvoiceResponse), AppError> {
        // Insert order
        let new_order = orders::ActiveModel {
            patient_name: Set(req.patient_name.clone()),
            order_date: Set(req.order_date),
            total_amount: Set(req.total_amount),
            description: Set(req.description.clone()),
            created_by: Set(Some(req.created_by)),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        // Generate a simple transaction ID
        let transaction_id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        // Create invoice service
        let invoice_service = InvoicesService::new(self.db.clone());

        // Create invoice for the order
        let invoice_req = CreateInvoiceRequest {
            order_id: new_order.order_id,
            transaction_id,
            invoice_date: new_order.order_date,
            total_amount: new_order.total_amount,
            description: new_order.description.clone(),
        };

        let invoice_model = invoice_service.create_invoice(invoice_req).await?;

        let invoice_response = InvoiceResponse {
            invoice_id: invoice_model.invoice_id,
            order_id: invoice_model.order_id,
            transaction_id: invoice_model.transaction_id,
            invoice_date: invoice_model.invoice_date,
            total_amount: invoice_model.total_amount,
            description: invoice_model.description,
        };

        let order_date = new_order.order_date;
        let first_day_of_month = NaiveDate::from_ymd_opt(order_date.year(), order_date.month(), 1)
            .expect("invalid date");

        let reports_service = ReportsService::new(self.db.clone());
        if let Err(e) = reports_service.generate_monthly_report(first_day_of_month).await {
            tracing::error!("Failed to auto-update monthly report: {}", e);
        }

        // Return both order and invoice
        Ok((
            CreateOrderResponse {
                order_id: new_order.order_id,
                patient_name: new_order.patient_name,
                order_date: new_order.order_date,
                total_amount: new_order.total_amount,
                description: new_order.description,
                created_by: new_order.created_by,
            },
            invoice_response,
        ))
    }

    /// Fetch all orders
    pub async fn get_orders(&self) -> Result<Vec<AllOrderResponse>, AppError> {
        let orders = orders::Entity::find()
            .all(&self.db)
            .await
            .map_err(AppError::from)?; // convert DbErr to AppError

        let response = orders
            .into_iter()
            .map(|order| AllOrderResponse {
                order_id: order.order_id,
                patient_name: order.patient_name,
                order_date: order.order_date,
                total_amount: order.total_amount,
                description: order.description,
                created_by: order.created_by,
                modified_by: order.modified_by,
            })
            .collect();

        Ok(response)
    }

    /// Fetch single order by ID
    pub async fn get_order_by_id(&self, order_id: i32) -> Result<GetOrderResponse, AppError> {
        let order = orders::Entity::find()
            .filter(orders::Column::OrderId.eq(order_id))
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Order not found".into()))?;

        Ok(GetOrderResponse {
            order_id: order.order_id,
            patient_name: order.patient_name,
            order_date: order.order_date,
            total_amount: order.total_amount,
            description: order.description,
            created_by: order.created_by,
            modified_by: order.modified_by,
        })
    }

    /// Update order
    pub async fn update_order(
        &self,
        id: i32,
        req: UpdateOrderRequest,
        user_id: i32,
    ) -> Result<(UpdateOrderResponse, Option<InvoiceResponse>), AppError> {
        use orders::Entity as Orders;

        // Fetch the existing order
        let existing = Orders::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found".into()))?;

        // Build active model for update
        let mut active: orders::ActiveModel = existing.into();

        if let Some(name) = req.patient_name.clone() {
            active.patient_name = Set(name);
        }
        if let Some(date) = req.order_date {
            active.order_date = Set(date);
        }
        if let Some(amount) = req.total_amount {
            active.total_amount = Set(amount);
        }
        if let Some(desc) = req.description.clone() {
            active.description = Set(desc);
        }

        active.modified_by = Set(Some(user_id));

        // Update the order in DB
        let updated_order = active.update(&self.db).await?;

        // Try to fetch related invoice
        let invoice = invoices::Entity::find()
            .filter(invoices::Column::OrderId.eq(id))
            .one(&self.db)
            .await?;


        // After updating the order in the database
        if let Some(updated_order) = updated_order {
            let order_date = updated_order.order_date;
            let first_day_of_month = NaiveDate::from_ymd_opt(order_date.year(), order_date.month(), 1)
                .expect("Invalid order date");

            let reports_service = ReportsService::new(self.db.clone());
            if let Err(e) = reports_service.generate_monthly_report(first_day_of_month).await {
                tracing::error!("Failed to auto-update monthly report after order update: {}", e);
            }
        }

        // If invoice exists, update total_amount and description to match updated order
        let updated_invoice = if let Some(mut invoice_model) = invoice {
            let mut invoice_active: invoices::ActiveModel = invoice_model.into();
            if let Some(amount) = req.total_amount {
                invoice_active.total_amount = Set(amount);
            }
            if let Some(desc) = req.description {
                invoice_active.description = Set(desc);
            }
            let updated = invoice_active.update(&self.db).await?;
            Some(InvoiceResponse {
                invoice_id: updated.invoice_id,
                order_id: updated.order_id,
                transaction_id: updated.transaction_id,
                invoice_date: updated.invoice_date,
                total_amount: updated.total_amount,
                description: updated.description,
            })
        } else {
            None
        };

        // Return both updated order and invoice
        Ok((
            UpdateOrderResponse {
                order_id: updated_order.order_id,
                patient_name: updated_order.patient_name,
                order_date: updated_order.order_date,
                total_amount: updated_order.total_amount,
                description: updated_order.description,
                created_by: updated_order.created_by,
                modified_by: updated_order.modified_by,
            },
            updated_invoice,
        ))
    }

    /// Delete order (cascade deletes invoice)
    pub async fn delete_order(&self, order_id: i32) -> Result<(), AppError> {
        let order: orders::ActiveModel = orders::Entity::find_by_id(order_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?
            .into();

        // SeaORM cascade delete handles invoice if FK is set
        order.delete(&self.db).await?;
        Ok(())
    }
}
