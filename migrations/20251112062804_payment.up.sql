-- Add up migration script here
CREATE TABLE payment (
    paymentid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    salesid UUID NOT NULL REFERENCES sales(salesid) ON DELETE CASCADE,
    amount NUMERIC(10,2) NOT NULL,
    method VARCHAR(50) NOT NULL,  
    date DATE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
