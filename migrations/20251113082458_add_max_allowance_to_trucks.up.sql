-- Add up migration script here
ALTER TABLE trucks
ADD COLUMN max_allowance DOUBLE PRECISION NOT NULL DEFAULT 4000;
