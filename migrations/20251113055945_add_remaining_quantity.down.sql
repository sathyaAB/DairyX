-- Add down migration script here
ALTER TABLE truck_load_products
DROP COLUMN remaining_quantity;
