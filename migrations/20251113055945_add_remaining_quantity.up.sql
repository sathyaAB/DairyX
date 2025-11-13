-- Add up migration script here
ALTER TABLE truck_load_products
ADD COLUMN remaining_quantity INT NOT NULL DEFAULT 0;
