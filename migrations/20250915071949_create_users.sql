-- Add migration script here
CREATE TABLE
    users (
        id UUID PRIMARY KEY,
        email TEXT UNIQUE NOT NULL,
        hashed_password TEXT NOT NULL,
        balance NUMERIC NOT NULL DEFAULT 0,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );