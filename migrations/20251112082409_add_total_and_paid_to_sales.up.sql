-- Add up migration script here
ALTER TABLE sales
ADD COLUMN total_amount DOUBLE PRECISION DEFAULT 0,
ADD COLUMN paid_amount DOUBLE PRECISION DEFAULT 0;
