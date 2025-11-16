use std::sync::Arc;

use axum::{http::{ StatusCode}, response::{IntoResponse}, routing::{ post}, Extension, Json, Router};

use validator::Validate;
use crate::models::UserRole;


use crate::{db::UserExt, dtos::{LoginUserDto, RegisterUserDto, Response, UserLoginResponseDto}, error::{HttpError},  utils::{password, token}, AppState};

pub fn auth_handler() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RegisterUserDto>
) -> Result<impl IntoResponse, HttpError> {
    println!("Request body: {:?}", body);

    // Validate input
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    // Hash password
    let hash_password = password::hash(&body.password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    // Set default role to Manager if not provided
    let role = body.role.unwrap_or(UserRole::Manager);

    // Save user in database
    let result = app_state.db_client
        .save_user(
            &body.first_name,
            &body.last_name,
            &body.email,
            &hash_password,
            role,
            body.address.as_deref(),
            body.city.as_deref(),
            body.district.as_deref(),
            body.contact_number.as_deref(),
        )
        .await;

    match result {
        Ok(_user) => Ok((StatusCode::CREATED, Json(Response {
            status: "success",
            message: "Registration successful!".to_string()
        }))),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    "Email already exists".to_string(),
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn login(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    println!("Login request body: {:?}", body);

    // Validate input
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    // Fetch user by email
    let user = app_state
        .db_client
        .get_user_by_email(&body.email)
        .await
        .map_err(|_| HttpError::bad_request("Invalid email or password".to_string()))?
        .ok_or_else(|| HttpError::bad_request("Invalid email or password".to_string()))?;

    // Verify password
    let is_valid = password::compare(&body.password, &user.password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    if !is_valid {
        return Err(HttpError::bad_request("Invalid email or password".to_string()));
    }

    // Generate JWT token
    let secret_key = app_state.env.jwt_secret.as_bytes();

    let token = token::create_token(&user.id.to_string(), &user.role.to_str(),secret_key,21600)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    println!("User '{}' logged in successfully", user.email);

    Ok((StatusCode::OK, Json(UserLoginResponseDto {
        status: "success".to_string(),
        token,
    })))
}



