-- Add migration script here
-- A postgreSQL migration script to create a 'transactions' table, necessary fields are: id, user_id, ticker, quantity, price, transaction_type, created_at, updated_at
CREATE TABLE
    transactions (
        id SERIAL PRIMARY KEY,
        user_id INT NOT NULL,
        ticker VARCHAR(10) NOT NULL,
        quantity INT NOT NULL,
        price DECIMAL(10, 2) NOT NULL,
        transaction_type VARCHAR(10) CHECK (transaction_type IN ('buy', 'sell')) NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );