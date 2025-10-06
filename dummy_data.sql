PRAGMA foreign_keys = ON;

-- Reset tables in dependency order
DELETE FROM attendance;
DELETE FROM module_tutor;
DELETE FROM lecturer_module;
DELETE FROM classes;
DELETE FROM module_students;
DELETE FROM modules;
DELETE FROM users;

-- Users
-- All users have password: password123 (simple_hash_17039136619706008904)
INSERT INTO users (name, surname, emailAddress, password, role, created_at, updated_at) VALUES
('John',   'Smith',   'john.smith@university.ac.za',  'simple_hash_17039136619706008904', 'lecturer', datetime('now'), datetime('now')),
('Sarah',  'Jones',   'sarah.jones@university.ac.za', 'simple_hash_17039136619706008904', 'lecturer', datetime('now'), datetime('now')),
('Mike',   'Williams','mike.williams@university.ac.za','simple_hash_17039136619706008904','tutor',    datetime('now'), datetime('now')),
('Lisa',   'Anderson','lisa.anderson@university.ac.za','simple_hash_17039136619706008904','tutor',    datetime('now'), datetime('now')),

('Emma',   'Brown',   'emma.brown@student.ac.za',     'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Oliver', 'Davis',   'oliver.davis@student.ac.za',   'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Ava',    'Miller',  'ava.miller@student.ac.za',     'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Liam',   'Wilson',  'liam.wilson@student.ac.za',    'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Sophia', 'Moore',   'sophia.moore@student.ac.za',   'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Noah',   'Taylor',  'noah.taylor@student.ac.za',    'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Isabella','Thomas', 'isabella.thomas@student.ac.za','simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('James',  'Jackson', 'james.jackson@student.ac.za',  'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Mia',    'White',   'mia.white@student.ac.za',      'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Lucas',  'Harris',  'lucas.harris@student.ac.za',   'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now')),
('Test',   'Student', 'test.student@student.ac.za',   'simple_hash_17039136619706008904', 'student', datetime('now'), datetime('now'));

-- Modules
INSERT INTO modules (moduleCode, moduleTitle, description, created_at, updated_at) VALUES
('CS301', 'Data Structures', 'Trees, graphs, dynamic programming', datetime('now'), datetime('now')),
('CS302', 'Web Development', 'Frontend + Backend development',      datetime('now'), datetime('now'));

-- Lecturer assignments
INSERT INTO lecturer_module (moduleCode, lecturerEmailAddress, created_at) VALUES
('CS301', 'john.smith@university.ac.za',  datetime('now')),
('CS302', 'john.smith@university.ac.za',  datetime('now'));

-- Tutor mappings (optional)
INSERT INTO module_tutor (moduleCode, tutorEmailAddress, created_at) VALUES
('CS301', 'mike.williams@university.ac.za', datetime('now')),
('CS302', 'lisa.anderson@university.ac.za', datetime('now'));

-- Enrolments (only enrolled students are considered in right-panel list and attendance below)
INSERT INTO module_students (moduleCode, studentEmailAddress, created_at) VALUES
('CS301', 'emma.brown@student.ac.za',   datetime('now')),
('CS301', 'oliver.davis@student.ac.za', datetime('now')),
('CS301', 'ava.miller@student.ac.za',   datetime('now')),
('CS301', 'liam.wilson@student.ac.za',  datetime('now')),
('CS302', 'sophia.moore@student.ac.za', datetime('now')),
('CS302', 'noah.taylor@student.ac.za',  datetime('now')),
('CS302', 'isabella.thomas@student.ac.za', datetime('now')),
('CS302', 'james.jackson@student.ac.za',   datetime('now')),
('CS302', 'test.student@student.ac.za',    datetime('now'));

-- Classes: 3 months × 4 weeks for each module
INSERT INTO classes (moduleCode, title, venue, description, recurring, date, time, status, created_at, updated_at) VALUES
-- CS301 current and previous two months (approx W1..W4)
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','+6 days'),  '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','+13 days'), '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','+20 days'), '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','+27 days'), '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','-1 months','+6 days'),  '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','-1 months','+13 days'), '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','-1 months','+20 days'), '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','-1 months','+27 days'), '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','-2 months','+6 days'),  '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','-2 months','+13 days'), '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','-2 months','+20 days'), '10:00','completed', datetime('now'), datetime('now')),
('CS301','DS Lecture','Hall A','Lecture','weekly', date('now','start of month','-2 months','+27 days'), '10:00','completed', datetime('now'), datetime('now')),
-- CS302 current and previous two months (approx W1..W4)
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','+5 days'),  '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','+12 days'), '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','+19 days'), '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','+26 days'), '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','-1 months','+5 days'),  '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','-1 months','+12 days'), '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','-1 months','+19 days'), '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','-1 months','+26 days'), '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','-2 months','+5 days'),  '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','-2 months','+12 days'), '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','-2 months','+19 days'), '14:00','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Lecture','Hall B','Lecture','weekly', date('now','start of month','-2 months','+26 days'), '14:00','completed', datetime('now'), datetime('now'));

-- Attendance (only for enrolled students)
-- CS301
INSERT INTO attendance (studentID, classID, status, recorded_at, notes)
SELECT u.userID, c.classID,
       CASE ((u.userID + c.classID) % 5)
            WHEN 0 THEN 'late'
            WHEN 1 THEN 'absent'
            ELSE 'present'
       END,
       datetime(c.date || ' ' || c.time),
       NULL
FROM users u
JOIN module_students ms ON ms.studentEmailAddress = u.emailAddress AND ms.moduleCode = 'CS301'
JOIN classes c ON c.moduleCode = 'CS301';

-- CS302
INSERT INTO attendance (studentID, classID, status, recorded_at, notes)
SELECT u.userID, c.classID,
       CASE ((u.userID + c.classID) % 4)
            WHEN 0 THEN 'late'
            WHEN 1 THEN 'absent'
            ELSE 'present'
       END,
       datetime(c.date || ' ' || c.time),
       NULL
FROM users u
JOIN module_students ms ON ms.studentEmailAddress = u.emailAddress AND ms.moduleCode = 'CS302'
JOIN classes c ON c.moduleCode = 'CS302';

-- ==========================
-- Variance data to change weekly/monthly averages
-- CS301: add a very low-attendance class in current month (W2)
INSERT INTO classes (moduleCode, title, venue, description, recurring, date, time, status, created_at, updated_at) VALUES
('CS301','DS Extra Low Attendance','Hall A','Test low attendance','single', date('now','start of month','+9 days'), '09:00','completed', datetime('now'), datetime('now'));

INSERT INTO attendance (studentID, classID, status, recorded_at, notes)
SELECT u.userID, c.classID,
       CASE WHEN (u.userID % 2)=0 THEN 'absent' ELSE 'late' END,
       datetime(c.date || ' ' || c.time),
       NULL
FROM users u
JOIN module_students ms ON ms.studentEmailAddress = u.emailAddress AND ms.moduleCode = 'CS301'
JOIN classes c ON c.moduleCode='CS301' AND c.date = date('now','start of month','+9 days') AND c.time='09:00';

-- CS301: add a very high-attendance class last month (W4)
INSERT INTO classes (moduleCode, title, venue, description, recurring, date, time, status, created_at, updated_at) VALUES
('CS301','DS Extra High Attendance','Hall A','Test high attendance','single', date('now','start of month','-1 months','+25 days'), '09:00','completed', datetime('now'), datetime('now'));

INSERT INTO attendance (studentID, classID, status, recorded_at, notes)
SELECT u.userID, c.classID,
       'present',
       datetime(c.date || ' ' || c.time),
       NULL
FROM users u
JOIN module_students ms ON ms.studentEmailAddress = u.emailAddress AND ms.moduleCode = 'CS301'
JOIN classes c ON c.moduleCode='CS301' AND c.date = date('now','start of month','-1 months','+25 days') AND c.time='09:00';

-- CS302: add mixed attendance class two months ago (W3)
INSERT INTO classes (moduleCode, title, venue, description, recurring, date, time, status, created_at, updated_at) VALUES
('CS302','Web Dev Mixed','Hall B','Mixed attendance','single', date('now','start of month','-2 months','+18 days'), '15:00','completed', datetime('now'), datetime('now'));

INSERT INTO attendance (studentID, classID, status, recorded_at, notes)
SELECT u.userID, c.classID,
       CASE ((u.userID) % 3)
            WHEN 0 THEN 'absent'
            WHEN 1 THEN 'late'
            ELSE 'present'
       END,
       datetime(c.date || ' ' || c.time),
       NULL
FROM users u
JOIN module_students ms ON ms.studentEmailAddress = u.emailAddress AND ms.moduleCode = 'CS302'
JOIN classes c ON c.moduleCode='CS302' AND c.date = date('now','start of month','-2 months','+18 days') AND c.time='15:00';

-- ==========================
-- Absent Today metric test: add classes today with some absentees/lates
INSERT INTO classes (moduleCode, title, venue, description, recurring, date, time, status, created_at, updated_at) VALUES
('CS301','DS Today','Hall A','Today session','single', date('now'), '10:30','completed', datetime('now'), datetime('now')),
('CS302','Web Dev Today','Hall B','Today session','single', date('now'), '11:30','completed', datetime('now'), datetime('now'));

-- CS301 today: 2 absent, 1 late, rest present
INSERT INTO attendance (studentID, classID, status, recorded_at, notes)
SELECT u.userID, c.classID,
       CASE ((u.userID) % 6)
            WHEN 0 THEN 'absent'
            WHEN 3 THEN 'late'
            ELSE 'present'
       END,
       datetime(c.date || ' ' || c.time),
       NULL
FROM users u
JOIN module_students ms ON ms.studentEmailAddress = u.emailAddress AND ms.moduleCode = 'CS301'
JOIN classes c ON c.moduleCode='CS301' AND c.date = date('now') AND c.time='10:30';

-- CS302 today: at least 1 absent
INSERT INTO attendance (studentID, classID, status, recorded_at, notes)
SELECT u.userID, c.classID,
       CASE WHEN (u.userID % 5)=0 THEN 'absent' ELSE 'present' END,
       datetime(c.date || ' ' || c.time),
       NULL
FROM users u
JOIN module_students ms ON ms.studentEmailAddress = u.emailAddress AND ms.moduleCode = 'CS302'
JOIN classes c ON c.moduleCode='CS302' AND c.date = date('now') AND c.time='11:30';

