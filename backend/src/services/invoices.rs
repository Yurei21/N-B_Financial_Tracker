use crate::errors::AppError;

pub async fn list_invoices() -> Result<(), AppError> {
    Ok(())
}

pub async fn get_invoice(_id: i32) -> Result<(), AppError> {
    Ok(())
}
