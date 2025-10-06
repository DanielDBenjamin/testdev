-- Add down migration script here
ALTER TABLE users ADD COLUMN university TEXT NOT NULL DEFAULT 'University of Example';