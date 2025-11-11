-- Add up migration script here
CREATE TABLE delivery_product (
    deliveryid UUID NOT NULL REFERENCES deliveries(deliveryid) ON DELETE CASCADE,
    productid UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INT NOT NULL CHECK (quantity > 0),
    PRIMARY KEY (deliveryid, productid)
);
