-- Add up migration script here
CREATE TABLE truck_load_products (
    truckloadid UUID NOT NULL REFERENCES truck_loads(truckloadid) ON DELETE CASCADE,
    productid UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INT NOT NULL CHECK (quantity > 0),
    PRIMARY KEY (truckloadid, productid)
);
