-- Add migration script here
CREATE TABLE holdings (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    ticker VARCHAR(10) NOT NULL,
    quantity INT NOT NULL,
    average_price NUMERIC(20, 10) NOT NULL,
    UNIQUE(user_id, ticker)
);