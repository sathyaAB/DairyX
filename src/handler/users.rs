use axum::{
    extract::{Path, Query, Extension},
    Json,
};
use std::sync::Arc;
use crate::dtos::{FilterUserDto, UserListResponseDto, RequestQueryDto};
use crate::error::HttpError;
use crate::AppState;
use axum::routing::get;
use axum::Router;
use crate::db::UserExt; 


pub fn users_handler() -> Router {
    Router::new()
        // Get a single user by ID
        .route("/:id", get(get_user))
        
        // Get list of users (with pagination query params)
        .route("/", get(get_users))
        
        // // Update user first & last name
        // .route("/:id/name", put(update_user_name))
        
        // // Update user role
        // .route("/:id/role", put(update_user_role))
        
        // // Update user password
        // .route("/:id/password", put(update_user_password))
}


pub async fn get_user(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<FilterUserDto>, HttpError> {
    let user_uuid = uuid::Uuid::parse_str(&user_id)
        .map_err(|_| HttpError::bad_request("Invalid user ID".to_string()))?;

    let user = app_state.db_client
        .get_user(Some(user_uuid), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or(HttpError::bad_request("User not found".to_string()))?;

    Ok(Json(FilterUserDto::filter_user(&user)))
}

// Get list of users with pagination
pub async fn get_users(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params): Query<RequestQueryDto>,
) -> Result<Json<UserListResponseDto>, HttpError> {
    let page = params.page.unwrap_or(1) as u32;
    let limit = params.limit.unwrap_or(20) as u32;

    let users = app_state.db_client
        .get_users(page , limit as usize)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let total = app_state.db_client
        .get_user_count()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(UserListResponseDto {
        status: "success".to_string(),
        users: users.iter().map(|u| FilterUserDto::filter_user(u)).collect(),
        results: total,
    }))
}
