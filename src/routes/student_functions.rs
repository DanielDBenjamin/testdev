use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::database::init_db_pool;
#[cfg(feature = "ssr")]
use chrono::Utc;

// Student enrollment data structures
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnrollStudentRequest {
    pub student_email: String,
    pub module_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentInfo {
    pub user_id: i64,
    pub name: String,
    pub surname: String,
    pub email_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnrollmentResponse {
    pub success: bool,
    pub message: String,
    pub student: Option<StudentInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentsListResponse {
    pub success: bool,
    pub message: String,
    pub students: Vec<StudentInfo>,
}

// Enroll a single student in a module
#[server(EnrollStudent, "/api")]
pub async fn enroll_student(
    request: EnrollStudentRequest,
) -> Result<EnrollmentResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    // Check if student exists
    let student = sqlx::query_as::<_, (i64, String, String, String)>(
        "SELECT userID, name, surname, emailAddress FROM users WHERE emailAddress = ? AND role = 'student'"
    )
    .bind(&request.student_email)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    let student = match student {
        Some(s) => s,
        None => return Ok(EnrollmentResponse {
            success: false,
            message: "Student not found with this email address".to_string(),
            student: None,
        }),
    };

    // Check if module exists
    let module_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM modules WHERE moduleCode = ?)"
    )
    .bind(&request.module_code)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    if !module_exists {
        return Ok(EnrollmentResponse {
            success: false,
            message: "Module not found".to_string(),
            student: None,
        });
    }

    // Check if already enrolled
    let already_enrolled = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM module_students WHERE moduleCode = ? AND studentEmailAddress = ?)"
    )
    .bind(&request.module_code)
    .bind(&request.student_email)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    if already_enrolled {
        return Ok(EnrollmentResponse {
            success: false,
            message: "Student is already enrolled in this module".to_string(),
            student: None,
        });
    }

    // Enroll student
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO module_students (moduleCode, studentEmailAddress, created_at) VALUES (?, ?, ?)"
    )
    .bind(&request.module_code)
    .bind(&request.student_email)
    .bind(&now)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to enroll student: {}", e)))?;

    Ok(EnrollmentResponse {
        success: true,
        message: "Student enrolled successfully!".to_string(),
        student: Some(StudentInfo {
            user_id: student.0,
            name: student.1,
            surname: student.2,
            email_address: student.3,
        }),
    })
}

// Get all students enrolled in a module
#[server(GetModuleStudents, "/api")]
pub async fn get_module_students(
    module_code: String,
) -> Result<StudentsListResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    let students = sqlx::query_as::<_, (i64, String, String, String)>(
        r#"
        SELECT u.userID, u.name, u.surname, u.emailAddress
        FROM users u
        INNER JOIN module_students ms ON u.emailAddress = ms.studentEmailAddress
        WHERE ms.moduleCode = ?
        ORDER BY u.surname, u.name
        "#
    )
    .bind(&module_code)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(StudentsListResponse {
        success: true,
        message: "Students fetched successfully".to_string(),
        students: students.into_iter().map(|(id, name, surname, email)| StudentInfo {
            user_id: id,
            name,
            surname,
            email_address: email,
        }).collect(),
    })
}

// Remove a student from a module
#[server(UnenrollStudent, "/api")]
pub async fn unenroll_student(
    module_code: String,
    student_email: String,
) -> Result<EnrollmentResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    let result = sqlx::query(
        "DELETE FROM module_students WHERE moduleCode = ? AND studentEmailAddress = ?"
    )
    .bind(&module_code)
    .bind(&student_email)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to unenroll student: {}", e)))?;

    if result.rows_affected() == 0 {
        return Ok(EnrollmentResponse {
            success: false,
            message: "Student was not enrolled in this module".to_string(),
            student: None,
        });
    }

    Ok(EnrollmentResponse {
        success: true,
        message: "Student removed successfully!".to_string(),
        student: None,
    })
}

// Bulk enroll students from CSV data
#[server(BulkEnrollStudents, "/api")]
pub async fn bulk_enroll_students(
    module_code: String,
    student_emails: Vec<String>,
) -> Result<EnrollmentResponse, ServerFnError> {
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    let now = Utc::now().to_rfc3339();
    let mut enrolled_count = 0;
    let mut errors = Vec::new();

    for email in student_emails {
        let email = email.trim().to_lowercase();
        if email.is_empty() {
            continue;
        }

        // Check if student exists
        let student_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM users WHERE emailAddress = ? AND role = 'student')"
        )
        .bind(&email)
        .fetch_one(&pool)
        .await
        .unwrap_or(false);

        if !student_exists {
            errors.push(format!("{} (not found)", email));
            continue;
        }

        // Check if already enrolled
        let already_enrolled = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM module_students WHERE moduleCode = ? AND studentEmailAddress = ?)"
        )
        .bind(&module_code)
        .bind(&email)
        .fetch_one(&pool)
        .await
        .unwrap_or(false);

        if already_enrolled {
            continue; // Skip already enrolled students silently
        }

        // Enroll student
        match sqlx::query(
            "INSERT INTO module_students (moduleCode, studentEmailAddress, created_at) VALUES (?, ?, ?)"
        )
        .bind(&module_code)
        .bind(&email)
        .bind(&now)
        .execute(&pool)
        .await {
            Ok(_) => enrolled_count += 1,
            Err(_) => errors.push(format!("{} (enrollment failed)", email)),
        }
    }

    let message = if errors.is_empty() {
        format!("Successfully enrolled {} student(s)", enrolled_count)
    } else {
        format!("Enrolled {} student(s). Errors: {}", enrolled_count, errors.join(", "))
    };

    Ok(EnrollmentResponse {
        success: enrolled_count > 0,
        message,
        student: None,
    })
}