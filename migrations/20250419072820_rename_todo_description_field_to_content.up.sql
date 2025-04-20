-- Add up migration script here
ALTER TABLE todos
RENAME COLUMN description TO content;