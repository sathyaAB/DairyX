-- Add down migration script here
ALTER TABLE sales
DROP COLUMN total_amount,
DROP COLUMN paid_amount;
