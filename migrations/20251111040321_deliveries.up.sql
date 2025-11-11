-- Add up migration script here
CREATE TABLE deliveries (
    deliveryid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE NOT NULL,
    userid UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);
