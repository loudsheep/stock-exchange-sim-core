-- Add migration script here
-- Change column 'hashed_password' to 'password' in postgres
ALTER TABLE users RENAME COLUMN hashed_password TO password;