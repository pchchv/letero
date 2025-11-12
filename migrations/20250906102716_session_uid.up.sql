-- Add up migration script here

ALTER TABLE Sessions ALTER COLUMN Uid TYPE CHAR(11);