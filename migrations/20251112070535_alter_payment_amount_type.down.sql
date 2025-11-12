-- Add down migration script here
ALTER TABLE payment
ALTER COLUMN amount TYPE NUMERIC(10,2);
