#[cfg(feature = "ssr")]
use crate::database::{init_db_pool, create_user, authenticate_user, update_user_password_by_email, CreateUserRequest};
use crate::types::{RegisterData, LoginData, AuthResponse, ResetPasswordData, BasicResponse};
use leptos::prelude::*;

#[server(RegisterUser, "/api")]
pub async fn register_user(data: RegisterData) -> Result<AuthResponse, ServerFnError> {
    // Validate input
    if data.name.trim().is_empty() {
        return Ok(AuthResponse {
            success: false,
            message: "Name is required".to_string(),
            user: None,
        });
    }

    if data.surname.trim().is_empty() {
        return Ok(AuthResponse {
            success: false,
            message: "Surname is required".to_string(),
            user: None,
        });
    }

    if data.email.trim().is_empty() {
        return Ok(AuthResponse {
            success: false,
            message: "Email is required".to_string(),
            user: None,
        });
    }

    if data.password.len() < 6 {
        return Ok(AuthResponse {
            success: false,
            message: "Password must be at least 6 characters".to_string(),
            user: None,
        });
    }

    if data.password != data.confirm_password {
        return Ok(AuthResponse {
            success: false,
            message: "Passwords do not match".to_string(),
            user: None,
        });
    }

    if !["lecturer", "tutor", "student"].contains(&data.role.as_str()) {
        return Ok(AuthResponse {
            success: false,
            message: "Invalid role selected".to_string(),
            user: None,
        });
    }

    // Initialize database connection
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    // Create user request
    let create_request = CreateUserRequest {
        name: data.name.trim().to_string(),
        surname: data.surname.trim().to_string(),
        email: data.email.trim().to_lowercase(),
        password: data.password,
        role: data.role,
    };

    // Create user
    match create_user(&pool, create_request).await {
        Ok(user) => Ok(AuthResponse {
            success: true,
            message: "Account created successfully!".to_string(),
            user: Some(user),
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            message: e,
            user: None,
        }),
    }
}

#[server(LoginUser, "/api")]
pub async fn login_user(data: LoginData) -> Result<AuthResponse, ServerFnError> {
    // Validate input
    if data.email.trim().is_empty() {
        return Ok(AuthResponse {
            success: false,
            message: "Email is required".to_string(),
            user: None,
        });
    }

    if data.password.trim().is_empty() {
        return Ok(AuthResponse {
            success: false,
            message: "Password is required".to_string(),
            user: None,
        });
    }

    // Initialize database connection
    let pool = init_db_pool().await.map_err(|e| {
        ServerFnError::new(format!("Database connection failed: {}", e))
    })?;

    // Authenticate user
    match authenticate_user(&pool, &data.email.trim().to_lowercase(), &data.password).await {
        Ok(user) => Ok(AuthResponse {
            success: true,
            message: "Login successful!".to_string(),
            user: Some(user),
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            message: e,
            user: None,
        }),
    }
}

#[server(ResetPassword, "/api")]
pub async fn reset_password_fn(data: ResetPasswordData) -> Result<BasicResponse, ServerFnError> {
    if data.email.trim().is_empty() {
        return Ok(BasicResponse {
            success: false,
            message: "Email is required".to_string(),
        });
    }

    if data.new_password.len() < 6 {
        return Ok(BasicResponse {
            success: false,
            message: "Password must be at least 6 characters".to_string(),
        });
    }

    if data.new_password != data.confirm_password {
        return Ok(BasicResponse {
            success: false,
            message: "Passwords do not match".to_string(),
        });
    }

    let pool = init_db_pool().await.map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;

    match update_user_password_by_email(&pool, &data.email.trim().to_lowercase(), &data.new_password).await {
        Ok(_) => Ok(BasicResponse {
            success: true,
            message: "Password updated successfully. You can now sign in with your new password.".to_string(),
        }),
        Err(e) => Ok(BasicResponse {
            success: false,
            message: e,
        }),
    }
}
