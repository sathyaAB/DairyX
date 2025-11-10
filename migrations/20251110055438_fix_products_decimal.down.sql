-- Add down migration script here
ALTER TABLE products
    ALTER COLUMN price TYPE NUMERIC(10,2) USING price::NUMERIC(10,2),
    ALTER COLUMN commission TYPE NUMERIC(5,2) USING commission::NUMERIC(5,2);
