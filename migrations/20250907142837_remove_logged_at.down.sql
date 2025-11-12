-- Add down migration script here

ALTER TABLE Sessions ADD COLUMN LoggedAt TIMESTAMPTZ;