-- Add up migration script here
CREATE TABLE truck_loads (
    truckloadid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE NOT NULL,
    userid UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    truckid UUID NOT NULL REFERENCES trucks(truckid) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
