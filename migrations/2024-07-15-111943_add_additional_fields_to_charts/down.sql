-- This file should undo anything in `up.sql`
ALTER TABLE charts
DROP COLUMN day,
DROP COLUMN month,
DROP COLUMN year,
DROP COLUMN hour,
DROP COLUMN min,
DROP COLUMN lat,
DROP COLUMN lon,
DROP COLUMN tzone;