-- Add up migration script here
CREATE TABLE sales_product (
    salesid UUID NOT NULL REFERENCES sales(salesid) ON DELETE CASCADE,
    productid UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INT NOT NULL,
    PRIMARY KEY (salesid, productid)
);
