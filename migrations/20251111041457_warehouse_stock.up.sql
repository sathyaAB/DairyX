-- Add up migration script here
CREATE TABLE warehouse_stock (
    stockid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    productid UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INT NOT NULL CHECK (quantity >= 0)
);
