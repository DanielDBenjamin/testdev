use sqlx::SqlitePool;

/// Initialize all database tables matching the ERD
pub async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("📋 Creating database tables...");

    // Create users table (matching your ERD)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            userID INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            surname TEXT NOT NULL,
            emailAddress TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            university TEXT NOT NULL,
            role TEXT NOT NULL CHECK (role IN ('student', 'lecturer', 'tutor')),
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create modules table (matching your ERD)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS modules (
            moduleCode INTEGER PRIMARY KEY,
            moduleTitle TEXT NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create classes table (matching your ERD)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS classes (
            classID INTEGER PRIMARY KEY AUTOINCREMENT,
            moduleCode INTEGER NOT NULL,
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
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create lecturer_module relationship table (matching your ERD)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS lecturer_module (
            moduleCode INTEGER NOT NULL,
            lecturerEmailAddress TEXT NOT NULL,
            created_at TEXT NOT NULL,
            PRIMARY KEY (moduleCode, lecturerEmailAddress),
            FOREIGN KEY (moduleCode) REFERENCES modules (moduleCode),
            FOREIGN KEY (lecturerEmailAddress) REFERENCES users (emailAddress)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create module_tutor relationship table (matching your ERD)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS module_tutor (
            moduleCode INTEGER NOT NULL,
            tutorEmailAddress TEXT NOT NULL,
            created_at TEXT NOT NULL,
            PRIMARY KEY (moduleCode, tutorEmailAddress),
            FOREIGN KEY (moduleCode) REFERENCES modules (moduleCode),
            FOREIGN KEY (tutorEmailAddress) REFERENCES users (emailAddress)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create attendance table (matching your ERD)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS attendance (
            attendanceID INTEGER PRIMARY KEY AUTOINCREMENT,
            studentID INTEGER NOT NULL,
            classID INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'absent' CHECK (status IN ('present', 'absent', 'late', 'excused')),
            recorded_at TEXT,
            notes TEXT,
            FOREIGN KEY (studentID) REFERENCES users (userID),
            FOREIGN KEY (classID) REFERENCES classes (classID),
            UNIQUE(studentID, classID)
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("✅ Database tables created successfully!");
    Ok(())
}

/// Verify that the database tables exist and are accessible
pub async fn test_database_structure(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("🧪 Testing database structure...");

    // Test that we can query each table (should return 0 rows initially)
    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    
    let module_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM modules")
        .fetch_one(pool)
        .await?;

    let class_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM classes")
        .fetch_one(pool)
        .await?;

    println!("📊 Initial database state:");
    println!("   👥 Users: {}", user_count.0);
    println!("   📚 Modules: {}", module_count.0);
    println!("   🎓 Classes: {}", class_count.0);
    println!("✅ Database structure test successful!");
    
    Ok(())
}