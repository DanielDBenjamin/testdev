use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::env;
use std::path::Path;

pub type DbPool = Pool<Sqlite>;

/// Initialize the SQLite database connection pool
pub async fn init_db_pool() -> Result<DbPool, sqlx::Error> {
    // Get database URL from environment or use default
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:clock_it.db".to_string());

    println!("🔗 Connecting to database: {}", database_url);

    // Extract the file path from the URL for checking
    let db_path = database_url
        .strip_prefix("sqlite:")
        .unwrap_or(&database_url);

    // Check if we can access the directory where the database file should be
    if let Some(parent_dir) = Path::new(db_path).parent() {
        if !parent_dir.exists() {
            println!("📁 Creating database directory: {:?}", parent_dir);
            std::fs::create_dir_all(parent_dir).map_err(|e| {
                sqlx::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!("Cannot create database directory: {}", e),
                ))
            })?;
        }
    }

    // Create connection pool with create-if-missing option
    let connection_string = format!("{}?mode=rwc", database_url);
    println!("🔗 Using connection string: {}", connection_string);

    let pool = SqlitePool::connect(&connection_string).await?;

    println!("✅ Database connection successful!");

    Ok(pool)
}

/// Test the database connection
pub async fn test_db_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    println!("🧪 Testing database connection...");

    // Simple query to test connection
    let result: (i64,) = sqlx::query_as("SELECT 1").fetch_one(pool).await?;

    if result.0 == 1 {
        println!("✅ Database connection test successful!");
        Ok(())
    } else {
        println!("❌ Database connection test failed!");
        Err(sqlx::Error::RowNotFound)
    }
}
