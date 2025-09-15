-- Add migration script here
-- Change column 'hashed_password' to 'password'
ALTER TABLE users
    ALTER COLUMN hashed_password RENAME TO password;