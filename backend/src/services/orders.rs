use sea_orm::{
    ActiveModelTrait, EntityTrait, QueryOrder, Set, DatabaseConnection,
    ColumnTrait, QueryFilter,
};
use chrono::NaiveDate;
use crate::entities::{orders, invoices};
use crate::errors::AppError;
use crate::utils::generate_transaction_id; // utility for invoice ID (we’ll add this below)

/// Create a new order and auto-generate its invoice
pub async fn create_order(
    db: &DatabaseConnection,
    patient_name: String,
    order_date: NaiveDate,
    total_amount: f64,
    description: String,
    created_by: Option<i32>,
) -> Result<orders::Model, AppError> {
    // Save order
    let new_order = orders::ActiveModel {
        patient_name: Set(patient_name.clone()),
        order_date: Set(order_date),
        total_amount: Set(total_amount),
        description: Set(description.clone()),
        created_by: Set(created_by),
        modified_by: Set(created_by),
        ..Default::default()
    };

    let order = new_order.insert(db).await?;

    // Generate invoice
    let transaction_id = generate_transaction_id();
    let invoice_date = chrono::Local::now().naive_local().date();

    let new_invoice = invoices::ActiveModel {
        order_id: Set(order.order_id),
        transaction_id: Set(transaction_id),
        invoice_date: Set(invoice_date),
        total_amount: Set(order.total_amount),
        description: Set(order.description.clone()),
        ..Default::default()
    };

    new_invoice.insert(db).await?;

    Ok(order)
}

/// Retrieve all orders (sorted by date descending)
pub async fn get_all_orders(db: &DatabaseConnection) -> Result<Vec<orders::Model>, AppError> {
    let list = orders::Entity::find()
        .order_by_desc(orders::Column::OrderDate)
        .all(db)
        .await?;
    Ok(list)
}

/// Update order by ID
pub async fn update_order(
    db: &DatabaseConnection,
    order_id: i32,
    patient_name: String,
    order_date: NaiveDate,
    total_amount: f64,
    description: String,
    modified_by: Option<i32>,
) -> Result<orders::Model, AppError> {
    let mut order = orders::Entity::find_by_id(order_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    order.patient_name = patient_name;
    order.order_date = order_date;
    order.total_amount = total_amount;
    order.description = description;

    let mut active: orders::ActiveModel = order.into();
    active.modified_by = Set(modified_by);

    Ok(active.update(db).await?)
}

/// Delete order and cascade delete its invoice
pub async fn delete_order(db: &DatabaseConnection, order_id: i32) -> Result<(), AppError> {
    let order = orders::Entity::find_by_id(order_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let active: orders::ActiveModel = order.into();
    active.delete(db).await?;
    Ok(())
}
