-- Add up migration script here

ALTER TABLE Users ADD COLUMN salt CHAR(32);