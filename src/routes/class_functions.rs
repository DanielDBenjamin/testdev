use leptos::prelude::*;
use crate::database::classes::{ Class, CreateClassRequest, UpdateClassRequest };
#[cfg(feature = "ssr")]
use crate::database::{init_db_pool, classes::{create_class, get_module_classes, get_lecturer_classes, delete_class, update_class}};


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClassResponse {
    pub success: bool,
    pub message: String,
    pub class: Option<Class>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClassesListResponse {
    pub success: bool,
    pub message: String,
    pub classes: Vec<Class>,
}

/// Create a new class
#[server(CreateClass, "/api")]
pub async fn create_class_fn(
    module_code: String,
    title: String,
    venue: Option<String>,
    description: Option<String>,
    recurring: Option<String>,
    date: String,
    time: String,
) -> Result<ClassResponse, ServerFnError> {
    // Add logging
    println!("Creating class for module: '{}'", module_code);
    
    if title.trim().is_empty() {
        return Ok(ClassResponse {
            success: false,
            message: "Class title is required".to_string(),
            class: None,
        });
    }

    if date.trim().is_empty() || time.trim().is_empty() {
        return Ok(ClassResponse {
            success: false,
            message: "Date and time are required".to_string(),
            class: None,
        });
    }

    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;
    
    // Verify the module exists
    let module_exists = sqlx::query("SELECT 1 FROM modules WHERE moduleCode = ?")
        .bind(&module_code)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to check module: {}", e)))?;
    
    if module_exists.is_none() {
        println!("Module '{}' not found in database!", module_code);
        return Ok(ClassResponse {
            success: false,
            message: format!("Module '{}' does not exist", module_code),
            class: None,
        });
    }

    let request = CreateClassRequest {
        module_code,
        title: title.trim().to_string(),
        venue: venue.filter(|s| !s.trim().is_empty()),
        description: description.filter(|s| !s.trim().is_empty()),
        recurring: recurring.filter(|s| !s.trim().is_empty() && s != "No repeat"),
        date,
        time,
    };

    match create_class(&pool, request).await {
        Ok(class) => Ok(ClassResponse {
            success: true,
            message: "Class created successfully!".to_string(),
            class: Some(class),
        }),
        Err(e) => {
            println!("Failed to create class: {}", e);
            Ok(ClassResponse {
                success: false,
                message: e,
                class: None,
            })
        }
    }
}

/// Get all classes for a module
#[server(GetModuleClasses, "/api")]
pub async fn get_module_classes_fn(
    module_code: String,
) -> Result<ClassesListResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    match get_module_classes(&pool, &module_code).await {
        Ok(classes) => Ok(ClassesListResponse {
            success: true,
            message: "Classes fetched successfully".to_string(),
            classes,
        }),
        Err(e) => Ok(ClassesListResponse {
            success: false,
            message: e,
            classes: vec![],
        }),
    }
}

/// Delete a class
#[server(DeleteClass, "/api")]
pub async fn delete_class_fn(
    class_id: i64,
) -> Result<ClassResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    match delete_class(&pool, class_id).await {
        Ok(_) => Ok(ClassResponse {
            success: true,
            message: "Class deleted successfully!".to_string(),
            class: None,
        }),
        Err(e) => Ok(ClassResponse {
            success: false,
            message: e,
            class: None,
        }),
    }
}

/// Get all classes for a lecturer
#[server(GetLecturerClasses, "/api")]
pub async fn get_lecturer_classes_fn(
    lecturer_email: String,
) -> Result<ClassesListResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    match get_lecturer_classes(&pool, &lecturer_email).await {
        Ok(classes) => Ok(ClassesListResponse {
            success: true,
            message: "Classes fetched successfully".to_string(),
            classes,
        }),
        Err(e) => Ok(ClassesListResponse {
            success: false,
            message: e,
            classes: vec![],
        }),
    }
}

/// Get a single class by ID
#[server(GetClass, "/api")]
pub async fn get_class_fn(
    class_id: i64,
) -> Result<ClassResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    let class = sqlx::query_as::<_, crate::database::classes::DbClass>(
        "SELECT * FROM classes WHERE classID = ?"
    )
    .bind(class_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    match class {
        Some(c) => Ok(ClassResponse {
            success: true,
            message: "Class found".to_string(),
            class: Some(c.into()),
        }),
        None => Ok(ClassResponse {
            success: false,
            message: "Class not found".to_string(),
            class: None,
        }),
    }
}

/// Update a class
#[server(UpdateClassFn, "/api")]
pub async fn update_class_fn(
    class_id: i64,
    title: String,
    description: Option<String>,
    date: String,
    time: String,
    venue: Option<String>,
    recurring: Option<String>,
) -> Result<ClassResponse, ServerFnError> {
    if title.trim().is_empty() {
        return Ok(ClassResponse {
            success: false,
            message: "Class title is required".to_string(),
            class: None,
        });
    }

    if date.trim().is_empty() {
        return Ok(ClassResponse {
            success: false,
            message: "Class date is required".to_string(),
            class: None,
        });
    }

    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    let request = UpdateClassRequest {
        title: title.trim().to_string(),
        description: description.filter(|s| !s.trim().is_empty()),
        date: date.trim().to_string(),
        time: time.trim().to_string(),
        duration: 60,
        venue: venue.filter(|s| !s.trim().is_empty()),
        recurring: recurring.filter(|s| !s.trim().is_empty()),
    };

    match update_class(&pool, class_id, request).await {
        Ok(class) => Ok(ClassResponse {
            success: true,
            message: "Class updated successfully!".to_string(),
            class: Some(class),
        }),
        Err(e) => Ok(ClassResponse {
            success: false,
            message: e,
            class: None,
        }),
    }
}

/// Update class status
#[server(UpdateClassStatus, "/api")]
pub async fn update_class_status_fn(
    class_id: i64,
    status: String,
) -> Result<ClassResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        UPDATE classes 
        SET status = ?, updated_at = ?
        WHERE classID = ?
        "#,
    )
    .bind(&status)
    .bind(&now)
    .bind(class_id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to update status: {}", e)))?;

    let class = sqlx::query_as::<_, crate::database::classes::DbClass>(
        "SELECT * FROM classes WHERE classID = ?"
    )
    .bind(class_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch updated class: {}", e)))?;

    Ok(ClassResponse {
        success: true,
        message: format!("Class status updated to {}", status),
        class: Some(class.into()),
    })
}