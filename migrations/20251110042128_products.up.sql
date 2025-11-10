-- Add up migration script here
-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "products" (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name VARCHAR(100) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    unit_type VARCHAR(50) NOT NULL,
    commission DECIMAL(5, 2) DEFAULT 0.00,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX products_name_idx ON products (name);
