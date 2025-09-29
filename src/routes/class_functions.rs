use leptos::prelude::*;
use crate::database::classes::{Class, CreateClassRequest};

#[cfg(feature = "ssr")]
use crate::database::{init_db_pool, classes::{create_class, get_module_classes, delete_class}};

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