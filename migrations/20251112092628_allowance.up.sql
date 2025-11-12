-- Add up migration script here
-- Allowance table
CREATE TABLE allowance (
    allowanceid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE NOT NULL UNIQUE,
    amount DOUBLE PRECISION NOT NULL,
    notes TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Truck allowance table
CREATE TABLE truck_allowance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    allowanceid UUID NOT NULL REFERENCES allowance(allowanceid) ON DELETE CASCADE,
    truckid UUID NOT NULL REFERENCES trucks(truckid),
    amount DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
