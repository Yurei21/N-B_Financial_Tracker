use uuid::Uuid;

/// Generate a random transaction ID for invoices
pub fn generate_transaction_id() -> String {
    format!("TXN-{}", Uuid::new_v4())
}
