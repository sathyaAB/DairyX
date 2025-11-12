-- Add up migration script here
CREATE TABLE sales (
    salesid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    truckloadid UUID NOT NULL REFERENCES truck_loads(truckloadid) ON DELETE CASCADE,
    shopid UUID NOT NULL REFERENCES shops(shopid) ON DELETE CASCADE,
    date DATE NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
