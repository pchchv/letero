-- Add down migration script here

ALTER TABLE Users ALTER COLUMN CreatedAt DROP NOT NULL;