-- Add down migration script here

ALTER TABLE Sessions ALTER COLUMN Uid TYPE CHAR(8);