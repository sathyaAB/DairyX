-- Add up migration script here
CREATE TYPE user_role AS ENUM ('admin', 'manager', 'driver');
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    role user_role NOT NULL DEFAULT 'manager',
    address VARCHAR(255),
    city VARCHAR(100),
    district VARCHAR(100),
    contact_number VARCHAR(20),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX users_email_idx ON users (email);


