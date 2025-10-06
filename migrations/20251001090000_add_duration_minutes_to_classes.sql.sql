ALTER TABLE classes ADD COLUMN duration_minutes INTEGER NOT NULL DEFAULT 90;

-- Backfill existing rows with default value (SQLite applies default automatically for new rows).
UPDATE classes SET duration_minutes = COALESCE(duration_minutes, 90);
