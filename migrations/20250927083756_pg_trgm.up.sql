-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX idx_users_name_trgm ON users USING gin (name gin_trgm_ops);