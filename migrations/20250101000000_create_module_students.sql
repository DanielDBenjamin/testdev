CREATE TABLE IF NOT EXISTS module_students (
    moduleCode TEXT NOT NULL,
    studentEmailAddress TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (moduleCode, studentEmailAddress),
    FOREIGN KEY (moduleCode) REFERENCES modules (moduleCode) ON DELETE CASCADE,
    FOREIGN KEY (studentEmailAddress) REFERENCES users (emailAddress) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_module_students_module ON module_students(moduleCode);
CREATE INDEX IF NOT EXISTS idx_module_students_student ON module_students(studentEmailAddress);