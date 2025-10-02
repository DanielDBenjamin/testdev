use leptos::prelude::*;
use crate::database::classes::{ Class, CreateClassRequest, UpdateClassRequest };
use gloo_net::http::Request;
#[cfg(feature = "ssr")]
use crate::database::{init_db_pool, classes::{create_class, get_module_classes, get_lecturer_classes, delete_class, update_class}};
#[cfg(feature = "ssr")]
use chrono::{NaiveDate, Duration};

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

/// Create a new class (with optional recurring pattern)
#[server(CreateClass, "/api")]
pub async fn create_class_fn(
    module_code: String,
    title: String,
    venue: Option<String>,
    description: Option<String>,
    recurring: Option<String>,
    date: String,
    time: String,
    recurrence_count: Option<i32>, // How many instances to create
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
        module_code: module_code.clone(),
        title: title.trim().to_string(),
        venue: venue.as_ref().and_then(|s| if s.trim().is_empty() { None } else { Some(s.clone()) }),
        description: description.as_ref().and_then(|s| if s.trim().is_empty() { None } else { Some(s.clone()) }),
        recurring: recurring.clone().filter(|s| !s.trim().is_empty() && s != "No repeat"),
        date: date.clone(),
        time: time.clone(),
    };

    // Create the first class
    let first_class = match create_class(&pool, request).await {
        Ok(class) => class,
        Err(e) => {
            println!("Failed to create class: {}", e);
            return Ok(ClassResponse {
                success: false,
                message: e,
                class: None,
            });
        }
    };

    // If recurring, create additional instances
    if let Some(recur_pattern) = &recurring {
        if recur_pattern != "No repeat" {
            let count = recurrence_count.unwrap_or(8); // Default to 8 weeks if not specified
            
            // Parse the start date
            if let Ok(start_date) = NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                let interval = match recur_pattern.as_str() {
                    "Daily" => Duration::days(1),
                    "Weekly" => Duration::weeks(1),
                    "Monthly" => Duration::days(30), // Approximate
                    _ => Duration::weeks(1), // Default to weekly
                };

                // Create additional class instances
                for i in 1..count {
                    let next_date = start_date + (interval * i);
                    let next_date_str = next_date.format("%Y-%m-%d").to_string();
                    
                    let recurring_request = CreateClassRequest {
                        module_code: module_code.clone(),
                        title: title.trim().to_string(),
                        venue: venue.as_ref().and_then(|s| if s.trim().is_empty() { None } else { Some(s.clone()) }),
                        description: description.as_ref().and_then(|s| if s.trim().is_empty() { None } else { Some(s.clone()) }),
                        recurring: Some(recur_pattern.clone()),
                        date: next_date_str,
                        time: time.clone(),
                    };

                    // Create each recurring instance
                    if let Err(e) = create_class(&pool, recurring_request).await {
                        println!("Warning: Failed to create recurring instance {}: {}", i, e);
                        // Continue creating other instances even if one fails
                    }
                }
            }
        }
    }

    let message = if recurring.is_some() && recurring.as_ref().unwrap() != "No repeat" {
        let count = recurrence_count.unwrap_or(8);
        format!("Created {} recurring class instances successfully!", count)
    } else {
        "Class created successfully!".to_string()
    };

    Ok(ClassResponse {
        success: true,
        message,
        class: Some(first_class),
    })
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

/// Update all classes in a recurring series
#[server(UpdateRecurringSeries, "/api")]
pub async fn update_recurring_series_fn(
    module_code: String,
    original_title: String,
    original_recurring: String,
    new_title: String,
    new_description: Option<String>,
    new_venue: Option<String>,
    new_time: String,
) -> Result<ClassResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    let now = chrono::Utc::now().to_rfc3339();

    // Update all classes that match the original series
    let result = sqlx::query(
        r#"
        UPDATE classes 
        SET title = ?, description = ?, venue = ?, time = ?, updated_at = ?
        WHERE moduleCode = ? AND title = ? AND recurring = ? AND status = 'upcoming'
        "#,
    )
    .bind(&new_title)
    .bind(&new_description)
    .bind(&new_venue)
    .bind(&new_time)
    .bind(&now)
    .bind(&module_code)
    .bind(&original_title)
    .bind(&original_recurring)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to update series: {}", e)))?;

    let count = result.rows_affected();

    Ok(ClassResponse {
        success: true,
        message: format!("Updated {} classes in the series", count),
        class: None,
    })
}

/// Rewrite a recurring series when the recurrence pattern changes
#[server(RewriteRecurringSeries, "/api")]
pub async fn rewrite_recurring_series_fn(
    class_id: i64,
    module_code: String,
    original_title: String,
    original_recurring: Option<String>,
    new_title: String,
    new_description: Option<String>,
    new_venue: Option<String>,
    new_date: String,
    new_time: String,
    new_recurring: Option<String>,
    new_recurrence_count: Option<i32>,
) -> Result<ClassResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    // 1) Find all upcoming classes that belong to the original series
    let series_class_ids: Vec<i64> = if let Some(orig_rec) = &original_recurring {
        sqlx::query_scalar(
            r#"SELECT classID FROM classes
               WHERE moduleCode = ? AND title = ? AND recurring = ? AND status = 'upcoming'"#,
        )
        .bind(&module_code)
        .bind(&original_title)
        .bind(orig_rec)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to query original series: {}", e)))?
    } else {
        sqlx::query_scalar(
            r#"SELECT classID FROM classes
               WHERE moduleCode = ? AND title = ? AND recurring IS NULL AND status = 'upcoming'"#,
        )
        .bind(&module_code)
        .bind(&original_title)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to query original series: {}", e)))?
    };

    let total_in_series = series_class_ids.len();

    // 2) Update the currently edited class to the new details
    let update_request = UpdateClassRequest {
        title: new_title.clone(),
        description: new_description.clone(),
        date: new_date.clone(),
        time: new_time.clone(),
        duration: 60,
        venue: new_venue.clone(),
        recurring: new_recurring.clone(),
    };

    let updated_class = match update_class(&pool, class_id, update_request).await {
        Ok(class) => class,
        Err(e) => {
            return Ok(ClassResponse {
                success: false,
                message: e,
                class: None,
            })
        }
    };

    // 3) Remove all other upcoming classes from the original series (they will be recreated)
    for id in series_class_ids {
        if id != class_id {
            if let Err(e) = delete_class(&pool, id).await {
                // Continue but report failure in the message later
                leptos::logging::log!("Failed to delete class {} while rewriting series: {}", id, e);
            }
        }
    }

    // 4) Recreate the remaining instances using the new recurrence
    let mut created = 0usize;
    if let Some(recur) = &new_recurring {
        // If user provided a new recurrence count, use it; otherwise keep original series length
        let target_total: usize = new_recurrence_count
            .map(|c| c.max(1) as usize)
            .unwrap_or_else(|| total_in_series.max(1));

        if target_total > 1 {
            if let Ok(start_date) = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d") {
                let interval = match recur.as_str() {
                    "Daily" => Duration::days(1),
                    "Weekly" => Duration::weeks(1),
                    "Monthly" => Duration::days(30),
                    _ => Duration::weeks(1),
                };

                for i in 1..target_total {
                    let next_date = start_date + (interval * (i as i32));
                    let next_date_str = next_date.format("%Y-%m-%d").to_string();

                    let req = CreateClassRequest {
                        module_code: module_code.clone(),
                        title: new_title.clone(),
                        venue: new_venue.clone(),
                        description: new_description.clone(),
                        recurring: new_recurring.clone(),
                        date: next_date_str,
                        time: new_time.clone(),
                    };

                    match create_class(&pool, req).await {
                        Ok(_) => created += 1,
                        Err(e) => {
                            leptos::logging::log!(
                                "Warning: failed to create rewritten instance {}: {}",
                                i, e
                            );
                        }
                    }
                }
            }
        }
    }

    let msg = if let Some(r) = &new_recurring {
        let total_instances = created + 1; // include the edited class
        format!(
            "Series updated to '{}' recurrence. Now {} instance(s) in total.",
            r, total_instances
        )
    } else {
        "Series updated to 'No repeat'. Removed future instances.".to_string()
    };

    Ok(ClassResponse {
        success: true,
        message: msg,
        class: Some(updated_class),
    })
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

// Helper function to save a single instance
pub async fn save_single_instance(
    class_id: i32,
    class_data: web_sys::FormData,
) -> Result<(), ServerFnError> {
    let url = format!("/api/classes/{}", class_id);
    
    let response = Request::put(&url)
        .body(class_data)?
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Request failed: {}", e)))?;
    
    if response.ok() {
        Ok(())
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(ServerFnError::new(format!(
            "Failed to update class: {}",
            error_text
        )))
    }
}

// Helper function to save all future instances in a series
pub async fn save_recurring_series(
    series_id: String,
    class_date: String,
    class_data: web_sys::FormData,
) -> Result<(), ServerFnError> {
    let url = format!("/api/classes/recurring/{}/from/{}", series_id, class_date);
    
    let response = Request::put(&url)
        .body(class_data)?
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Request failed: {}", e)))?;
    
    if response.ok() {
        Ok(())
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(ServerFnError::new(format!(
            "Failed to update recurring classes: {}",
            error_text
        )))
    }
}
