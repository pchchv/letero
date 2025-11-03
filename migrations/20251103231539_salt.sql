-- Add up migration script here

ALTER TABLE Users ADD COLUMN salt CHAR(32);

-- Add down migration script here

ALTER TABLE Users DROP COLUMN salt;