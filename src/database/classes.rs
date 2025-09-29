#[cfg(feature = "ssr")]
use sqlx::SqlitePool;
#[cfg(feature = "ssr")]
use chrono::Utc;

use serde::{Deserialize, Serialize};

// Class types available for both client and server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub class_id: i64,
    pub module_code: String,
    pub title: String,
    pub venue: Option<String>,
    pub description: Option<String>,
    pub recurring: Option<String>,
    pub date: String,
    pub time: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClassRequest {
    pub module_code: String,
    pub title: String,
    pub venue: Option<String>,
    pub description: Option<String>,
    pub recurring: Option<String>,
    pub date: String,
    pub time: String,
}

// Server-side implementation
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg(feature = "ssr")]
#[derive(Debug, Clone, FromRow)]
struct DbClass {
    #[sqlx(rename = "classID")]
    class_id: i64,
    #[sqlx(rename = "moduleCode")]
    module_code: String,
    title: String,
    venue: Option<String>,
    description: Option<String>,
    recurring: Option<String>,
    date: String,
    time: String,
    status: String,
    created_at: String,
    updated_at: String,
}

#[cfg(feature = "ssr")]
impl From<DbClass> for Class {
    fn from(db: DbClass) -> Self {
        Class {
            class_id: db.class_id,
            module_code: db.module_code,
            title: db.title,
            venue: db.venue,
            description: db.description,
            recurring: db.recurring,
            date: db.date,
            time: db.time,
            status: db.status,
            created_at: db.created_at,
            updated_at: db.updated_at,
        }
    }
}

/// Create a new class
#[cfg(feature = "ssr")]
pub async fn create_class(
    pool: &SqlitePool,
    request: CreateClassRequest,
) -> Result<Class, String> {
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"
        INSERT INTO classes (moduleCode, title, venue, description, recurring, date, time, status, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, 'upcoming', ?, ?)
        "#,
    )
    .bind(&request.module_code)
    .bind(&request.title)
    .bind(&request.venue)
    .bind(&request.description)
    .bind(&request.recurring)
    .bind(&request.date)
    .bind(&request.time)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create class: {}", e))?;

    let class_id = result.last_insert_rowid();

    let class = sqlx::query_as::<_, DbClass>(
        "SELECT * FROM classes WHERE classID = ?"
    )
    .bind(class_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to fetch created class: {}", e))?;

    Ok(class.into())
}

/// Get all classes for a module
#[cfg(feature = "ssr")]
pub async fn get_module_classes(
    pool: &SqlitePool,
    module_code: &str,
) -> Result<Vec<Class>, String> {
    let classes = sqlx::query_as::<_, DbClass>(
        r#"
        SELECT * FROM classes 
        WHERE moduleCode = ?
        ORDER BY date, time
        "#,
    )
    .bind(module_code)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(classes.into_iter().map(|c| c.into()).collect())
}

/// Get all classes for a lecturer (across all their modules)
#[cfg(feature = "ssr")]
pub async fn get_lecturer_classes(
    pool: &SqlitePool,
    lecturer_email: &str,
) -> Result<Vec<Class>, String> {
    let classes = sqlx::query_as::<_, DbClass>(
        r#"
        SELECT c.* FROM classes c
        INNER JOIN lecturer_module lm ON c.moduleCode = lm.moduleCode
        WHERE lm.lecturerEmailAddress = ?
        ORDER BY c.date, c.time
        "#,
    )
    .bind(lecturer_email)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(classes.into_iter().map(|c| c.into()).collect())
}

/// Delete a class
#[cfg(feature = "ssr")]
pub async fn delete_class(
    pool: &SqlitePool,
    class_id: i64,
) -> Result<(), String> {
    sqlx::query("DELETE FROM attendance WHERE classID = ?")
        .bind(class_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete attendance records: {}", e))?;

    sqlx::query("DELETE FROM classes WHERE classID = ?")
        .bind(class_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete class: {}", e))?;

    Ok(())
}