ALTER TABLE class_sessions ADD COLUMN start_latitude REAL;
ALTER TABLE class_sessions ADD COLUMN start_longitude REAL;
ALTER TABLE class_sessions ADD COLUMN start_accuracy REAL;
ALTER TABLE class_sessions ADD COLUMN location_radius REAL;

ALTER TABLE attendance ADD COLUMN check_latitude REAL;
ALTER TABLE attendance ADD COLUMN check_longitude REAL;
ALTER TABLE attendance ADD COLUMN location_accuracy REAL;
