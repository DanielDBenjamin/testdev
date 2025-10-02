-- All users have password: password123
-- Hash generated using the simple_hash function in auth.rs

-- Insert dummy users (lecturers, tutors, and students)
INSERT INTO users (name, surname, emailAddress, password, role, created_at, updated_at) VALUES
('John', 'Smith', 'john.smith@university.ac.za', 'simple_hash_17039136619706008904', 'lecturer', datetime('now'), datetime('now')),
('Sarah', 'Jones', 'sarah.jones@university.ac.za', 'simple_hash_17039136619706008904', 'lecturer', datetime('now'), datetime('now')),
('Mike', 'Williams', 'mike.williams@university.ac.za', 'simple_hash_17039136619706008904', 'tutor', datetime('now'), datetime('now')),
('Lisa', 'Anderson', 'lisa.anderson@university.ac.za', 'simple_hash_17039136619706008904', 'tutor', datetime('now'), datetime('now')),

('Emma', 'Brown', 'emma.brown@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Oliver', 'Davis', 'oliver.davis@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Ava', 'Miller', 'ava.miller@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Liam', 'Wilson', 'liam.wilson@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Sophia', 'Moore', 'sophia.moore@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Noah', 'Taylor', 'noah.taylor@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Isabella', 'Thomas', 'isabella.thomas@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('James', 'Jackson', 'james.jackson@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Mia', 'White', 'mia.white@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Lucas', 'Harris', 'lucas.harris@student.ac.za', 'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now'));

-- Insert modules
INSERT INTO modules (moduleCode, moduleTitle, description, created_at, updated_at) VALUES
('CS301', 'Data Structures', 'Advanced data structures and algorithms including trees, graphs, and dynamic programming', datetime('now'), datetime('now')),
('CS302', 'Web Development', 'Modern web development using React, Node.js, and database integration', datetime('now'), datetime('now')),
('CS303', 'Machine Learning', 'Introduction to machine learning algorithms, neural networks, and practical applications', datetime('now'), datetime('now')),
('IS201', 'Database Systems', 'Relational database design, SQL, normalization, and transaction management', datetime('now'), datetime('now')),
('CS304', 'Operating Systems', 'Process management, memory allocation, file systems, and concurrency', datetime('now'), datetime('now'));

-- Link lecturers to modules
INSERT INTO lecturer_module (moduleCode, lecturerEmailAddress, created_at) VALUES
('CS301', 'john.smith@university.ac.za', datetime('now')),
('CS302', 'john.smith@university.ac.za', datetime('now')),
('CS303', 'sarah.jones@university.ac.za', datetime('now')),
('IS201', 'sarah.jones@university.ac.za', datetime('now')),
('CS304', 'john.smith@university.ac.za', datetime('now'));

-- Link tutors to modules
INSERT INTO module_tutor (moduleCode, tutorEmailAddress, created_at) VALUES
('CS301', 'mike.williams@university.ac.za', datetime('now')),
('CS302', 'mike.williams@university.ac.za', datetime('now')),
('CS303', 'lisa.anderson@university.ac.za', datetime('now'));

-- Insert classes (past and upcoming)
INSERT INTO classes (moduleCode, title, venue, description, recurring, date, time, status, created_at, updated_at) VALUES
-- Past classes (completed)
('CS301', 'Data Structures Lecture', 'Lecture Hall A', 'Introduction to trees and graphs', 'weekly', date('now', '-14 days'), '08:00', 'completed', datetime('now', '-14 days'), datetime('now')),
('CS301', 'Data Structures Practical', 'Lab 3', 'Implementing binary search trees', 'weekly', date('now', '-12 days'), '14:00', 'completed', datetime('now', '-12 days'), datetime('now')),
('CS302', 'Web Dev Lecture', 'Lecture Hall B', 'React hooks and state management', 'weekly', date('now', '-13 days'), '10:00', 'completed', datetime('now', '-13 days'), datetime('now')),
('CS303', 'ML Lecture', 'Lecture Hall C', 'Neural network basics', 'weekly', date('now', '-11 days'), '08:00', 'completed', datetime('now', '-11 days'), datetime('now')),

('CS301', 'Data Structures Lecture', 'Lecture Hall A', 'Advanced graph algorithms', 'weekly', date('now', '-7 days'), '08:00', 'completed', datetime('now', '-7 days'), datetime('now')),
('CS302', 'Web Dev Lab', 'Lab 1', 'Building REST APIs', 'weekly', date('now', '-6 days'), '14:00', 'completed', datetime('now', '-6 days'), datetime('now')),
('CS304', 'OS Lecture', 'Lecture Hall B', 'Process scheduling', 'weekly', date('now', '-7 days'), '14:00', 'completed', datetime('now', '-7 days'), datetime('now')),

('CS301', 'Data Structures Lecture', 'Lecture Hall A', 'Dynamic programming', 'weekly', date('now', '-2 days'), '08:00', 'completed', datetime('now', '-2 days'), datetime('now')),
('IS201', 'Database Lecture', 'Lecture Hall D', 'SQL joins and subqueries', 'weekly', date('now', '-3 days'), '12:00', 'completed', datetime('now', '-3 days'), datetime('now')),

-- Upcoming classes
('CS301', 'Data Structures Practical', 'Lab 3', 'Graph traversal algorithms', 'weekly', date('now', '+1 days'), '14:00', 'upcoming', datetime('now'), datetime('now')),
('CS302', 'Web Dev Lecture', 'Lecture Hall B', 'Frontend optimization', 'weekly', date('now', '+2 days'), '10:00', 'upcoming', datetime('now'), datetime('now')),
('CS303', 'ML Practical', 'Lab 2', 'Training your first neural network', 'weekly', date('now', '+3 days'), '10:00', 'upcoming', datetime('now'), datetime('now')),
('CS304', 'OS Lecture', 'Lecture Hall C', 'Memory management', 'weekly', date('now', '+4 days'), '10:00', 'upcoming', datetime('now'), datetime('now')),
('IS201', 'Database Lab', 'Lab 4', 'Database design project', 'weekly', date('now', '+5 days'), '14:00', 'upcoming', datetime('now'), datetime('now'));

-- Insert attendance records for completed classes
-- Class 1 (CS301 - 14 days ago) - classID will be 1
INSERT INTO attendance (studentID, classID, status, recorded_at, notes) VALUES
(5, 1, 'present', datetime('now', '-14 days', '+8 hours'), NULL),
(6, 1, 'present', datetime('now', '-14 days', '+8 hours'), NULL),
(7, 1, 'late', datetime('now', '-14 days', '+8 hours', '+15 minutes'), 'Arrived 15 min late'),
(8, 1, 'absent', NULL, NULL),
(9, 1, 'present', datetime('now', '-14 days', '+8 hours'), NULL),
(10, 1, 'present', datetime('now', '-14 days', '+8 hours'), NULL),
(11, 1, 'present', datetime('now', '-14 days', '+8 hours'), NULL);

-- Class 2 (CS301 Practical - 12 days ago) - classID will be 2
INSERT INTO attendance (studentID, classID, status, recorded_at, notes) VALUES
(5, 2, 'present', datetime('now', '-12 days', '+14 hours'), NULL),
(6, 2, 'absent', NULL, 'Sick'),
(7, 2, 'present', datetime('now', '-12 days', '+14 hours'), NULL),
(8, 2, 'present', datetime('now', '-12 days', '+14 hours'), NULL),
(9, 2, 'late', datetime('now', '-12 days', '+14 hours', '+10 minutes'), NULL),
(10, 2, 'present', datetime('now', '-12 days', '+14 hours'), NULL);

-- Class 3 (CS302 - 13 days ago) - classID will be 3
INSERT INTO attendance (studentID, classID, status, recorded_at, notes) VALUES
(5, 3, 'present', datetime('now', '-13 days', '+10 hours'), NULL),
(6, 3, 'present', datetime('now', '-13 days', '+10 hours'), NULL),
(7, 3, 'present', datetime('now', '-13 days', '+10 hours'), NULL),
(12, 3, 'late', datetime('now', '-13 days', '+10 hours', '+20 minutes'), NULL),
(13, 3, 'present', datetime('now', '-13 days', '+10 hours'), NULL);

-- Class 4 (CS303 - 11 days ago) - classID will be 4
INSERT INTO attendance (studentID, classID, status, recorded_at, notes) VALUES
(8, 4, 'present', datetime('now', '-11 days', '+8 hours'), NULL),
(9, 4, 'absent', NULL, NULL),
(11, 4, 'present', datetime('now', '-11 days', '+8 hours'), NULL),
(13, 4, 'present', datetime('now', '-11 days', '+8 hours'), NULL);

-- Class 5 (CS301 - 7 days ago) - classID will be 5
INSERT INTO attendance (studentID, classID, status, recorded_at, notes) VALUES
(5, 5, 'present', datetime('now', '-7 days', '+8 hours'), NULL),
(6, 5, 'present', datetime('now', '-7 days', '+8 hours'), NULL),
(7, 5, 'present', datetime('now', '-7 days', '+8 hours'), NULL),
(8, 5, 'absent', NULL, NULL),
(9, 5, 'late', datetime('now', '-7 days', '+8 hours', '+12 minutes'), NULL),
(10, 5, 'present', datetime('now', '-7 days', '+8 hours'), NULL),
(11, 5, 'present', datetime('now', '-7 days', '+8 hours'), NULL);

-- Class 6 (CS302 Lab - 6 days ago) - classID will be 6
INSERT INTO attendance (studentID, classID, status, recorded_at, notes) VALUES
(5, 6, 'present', datetime('now', '-6 days', '+14 hours'), NULL),
(6, 6, 'present', datetime('now', '-6 days', '+14 hours'), NULL),
(7, 6, 'absent', NULL, 'Medical appointment'),
(12, 6, 'present', datetime('now', '-6 days', '+14 hours'), NULL),
(13, 6, 'present', datetime('now', '-6 days', '+14 hours'), NULL);

-- Class 7 (CS304 - 7 days ago) - classID will be 7
INSERT INTO attendance (studentID, classID, status, recorded_at, notes) VALUES
(5, 7, 'present', datetime('now', '-7 days', '+14 hours'), NULL),
(6, 7, 'present', datetime('now', '-7 days', '+14 hours'), NULL),
(14, 7, 'late', datetime('now', '-7 days', '+14 hours', '+18 minutes'), NULL);

-- Class 8 (CS301 - 2 days ago) - classID will be 8
INSERT INTO attendance (studentID, classID, status, recorded_at, notes) VALUES
(5, 8, 'present', datetime('now', '-2 days', '+8 hours'), NULL),
(6, 8, 'present', datetime('now', '-2 days', '+8 hours'), NULL),
(7, 8, 'present', datetime('now', '-2 days', '+8 hours'), NULL),
(8, 8, 'present', datetime('now', '-2 days', '+8 hours'), NULL),
(9, 8, 'present', datetime('now', '-2 days', '+8 hours'), NULL),
(10, 8, 'late', datetime('now', '-2 days', '+8 hours', '+8 minutes'), NULL),
(11, 8, 'present', datetime('now', '-2 days', '+8 hours'), NULL);

-- Class 9 (IS201 - 3 days ago) - classID will be 9
INSERT INTO attendance (studentID, classID, status, recorded_at, notes) VALUES
(10, 9, 'present', datetime('now', '-3 days', '+12 hours'), NULL),
(11, 9, 'present', datetime('now', '-3 days', '+12 hours'), NULL),
(12, 9, 'absent', NULL, NULL);