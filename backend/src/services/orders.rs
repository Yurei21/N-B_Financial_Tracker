use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter, ActiveValue};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::{
    entities::{orders, invoices, users},
    errors::AppError,
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

impl OrdersService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_order(&self, req: CreateOrderRequest) -> Result<CreateOrderResponse, AppError> {
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

        Ok(CreateOrderResponse {
            order_id: new_order.order_id,
            patient_name: new_order.patient_name,
            order_date: new_order.order_date,
            total_amount: new_order.total_amount,
            description: new_order.description,
            created_by: new_order.created_by,
        })
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
    pub async fn update_order(&self, order_id: i32, req: UpdateOrderRequest, modified_by: i32) -> Result<orders::Model, AppError> {
        let mut order: orders::ActiveModel = orders::Entity::find_by_id(order_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)?
            .into();

        if let Some(name) = req.patient_name { order.patient_name = Set(name); }
        if let Some(date) = req.order_date { order.order_date = Set(date); }
        if let Some(amount) = req.total_amount { order.total_amount = Set(amount); }
        if let Some(desc) = req.description { order.description = Set(desc); }

        order.modified_by = Set(Some(modified_by));

        let updated = order.update(&self.db).await?;
        Ok(updated)
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
