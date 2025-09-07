-- Users Table
CREATE TABLE IF NOT EXISTS users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_name VARCHAR(255),
    password_hash TEXT NOT NULL
);

-- Orders Table
CREATE TABLE IF NOT EXISTS orders (
    order_id INTEGER PRIMARY KEY AUTOINCREMENT,
    patient_name TEXT NOT NULL,
    order_date TEXT NOT NULL DEFAULT (datetime('now')),
    total_amount REAL NOT NULL
);

-- Expenses Table
CREATE TABLE IF NOT EXISTS expenses (
    expense_id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    label TEXT,
    amount REAL NOT NULL,
    expense_date TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Invoices Table
CREATE TABLE IF NOT EXISTS invoices (
    invoice_id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER NOT NULL,
    transaction_id TEXT UNIQUE NOT NULL,
    invoice_date TEXT NOT NULL DEFAULT (datetime('now')),
    total_amount REAL NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(order_id) ON DELETE CASCADE
);
