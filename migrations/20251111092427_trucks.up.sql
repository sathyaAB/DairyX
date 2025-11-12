-- Add up migration script here
CREATE TABLE trucks (
    truckid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trucknumber VARCHAR(50) NOT NULL UNIQUE,
    model VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
