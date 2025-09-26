-- Drop tables in reverse order (to handle foreign key constraints)
DROP TABLE IF EXISTS attendance;
DROP TABLE IF EXISTS module_tutor;
DROP TABLE IF EXISTS lecturer_module;
DROP TABLE IF EXISTS classes;
DROP TABLE IF EXISTS modules;
DROP TABLE IF EXISTS users;