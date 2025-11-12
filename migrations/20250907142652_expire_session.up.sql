-- Add up migration script here

ALTER TABLE Sessions ADD COLUMN ExpiresAt TIMESTAMPTZ NOT NULL;