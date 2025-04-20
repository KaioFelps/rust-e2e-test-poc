-- Add down migration script here
ALTER TABLE todos
RENAME COLUMN content TO description;