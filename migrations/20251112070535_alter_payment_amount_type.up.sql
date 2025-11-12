-- Add up migration script here
ALTER TABLE payment
ALTER COLUMN amount TYPE DOUBLE PRECISION;
