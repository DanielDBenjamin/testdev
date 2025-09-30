-- Add migration script here
-- Create users table (matching your ERD)
CREATE TABLE users (
    userID INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    surname TEXT NOT NULL,
    emailAddress TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    university TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('student', 'lecturer', 'tutor')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create modules table (matching your ERD)
CREATE TABLE modules (
    moduleCode TEXT PRIMARY KEY,
    moduleTitle TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create classes table (matching your ERD)
CREATE TABLE classes (
    classID INTEGER PRIMARY KEY AUTOINCREMENT,
    moduleCode TEXT NOT NULL,
    title TEXT NOT NULL,
    venue TEXT,
    description TEXT,
    recurring TEXT,
    date TEXT NOT NULL,
    time TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'upcoming' CHECK (status IN ('upcoming', 'in_progress', 'completed', 'cancelled')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (moduleCode) REFERENCES modules (moduleCode)
);

-- Create lecturer_module relationship table (matching your ERD)
CREATE TABLE lecturer_module (
    moduleCode TEXT NOT NULL,
    lecturerEmailAddress TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (moduleCode, lecturerEmailAddress),
    FOREIGN KEY (moduleCode) REFERENCES modules (moduleCode),
    FOREIGN KEY (lecturerEmailAddress) REFERENCES users (emailAddress)
);

-- Create module_tutor relationship table (matching your ERD)
CREATE TABLE module_tutor (
    moduleCode TEXT NOT NULL,
    tutorEmailAddress TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (moduleCode, tutorEmailAddress),
    FOREIGN KEY (moduleCode) REFERENCES modules (moduleCode),
    FOREIGN KEY (tutorEmailAddress) REFERENCES users (emailAddress)
);

-- Create attendance table (matching your ERD)
CREATE TABLE attendance (
    attendanceID INTEGER PRIMARY KEY AUTOINCREMENT,
    studentID INTEGER NOT NULL,
    classID INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'absent' CHECK (status IN ('present', 'absent', 'late', 'excused')),
    recorded_at TEXT,
    notes TEXT,
    FOREIGN KEY (studentID) REFERENCES users (userID),
    FOREIGN KEY (classID) REFERENCES classes (classID),
    UNIQUE(studentID, classID)
);