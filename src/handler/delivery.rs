use axum::{
    extract::{Path, Extension},  
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::dtos::{CreateDeliveryDto, DeliveryResponseDto, DeliveryListResponseDto};
use crate::error::{HttpError, ErrorMessage};
use crate::db::{DBClient, DeliveryExt};
use crate::models::UserRole;
use crate::middleware::JWTAuthMiddeware;
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use uuid::Uuid;

pub fn delivery_handler() -> Router { 
    Router::new()
        .route("/create", post(create_delivery))
        .route("/history", get(get_delivery_history))
}

pub async fn create_delivery(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,  
    Extension(app_state): Extension<Arc<AppState>>,    
    Json(body): Json<CreateDeliveryDto>,
) -> Result<Json<DeliveryResponseDto>, HttpError> {
    // Check if user has Manager or Admin role
    if jwt_auth.user.role != UserRole::Manager && jwt_auth.user.role != UserRole::Admin {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    // Parse date
    let date = chrono::NaiveDate::parse_from_str(&body.date, "%Y-%m-%d")
        .map_err(|_| HttpError::bad_request("Invalid date format".to_string()))?;

    // Prepare products vector
    let products: Vec<(Uuid, i32)> = body
        .products
        .into_iter()
        .map(|p| (p.product_id, p.quantity))
        .collect();

    // Get user id from JWT token
    let user_id = jwt_auth.user.id;

    // Create delivery
    let delivery = app_state.db_client
        .create_delivery(user_id, date, products)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(DeliveryResponseDto {
        status: "success".to_string(),
        delivery,
    }))
}

pub async fn get_delivery_history(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<Json<DeliveryListResponseDto>, HttpError> {
    let user_id = jwt_auth.user.id;

    // Fetch all deliveries for the logged-in user
    let deliveries = app_state.db_client
        .get_deliveries_by_user(user_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(DeliveryListResponseDto {
        status: "success".to_string(),
        deliveries,
    }))
}