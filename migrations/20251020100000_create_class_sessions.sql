CREATE TABLE class_sessions (
    sessionID INTEGER PRIMARY KEY AUTOINCREMENT,
    classID INTEGER NOT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    started_by TEXT,
    FOREIGN KEY (classID) REFERENCES classes (classID)
);

CREATE INDEX idx_class_sessions_class ON class_sessions(classID);
CREATE INDEX idx_class_sessions_active ON class_sessions(classID, ended_at);
