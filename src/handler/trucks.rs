use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::dtos::{CreateTruckRequest, CreateTruckResponse};
use crate::error::{HttpError, ErrorMessage};
use crate::db::TruckExt;
use crate::middleware::JWTAuthMiddeware;
use crate::AppState;
use axum::routing::{post, get};
use axum::Router;

pub fn truck_handler() -> Router {
    Router::new()
        .route("/create", post(create_truck))
        .route("/all", get(get_all_trucks))
}

pub async fn create_truck(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateTruckRequest>,
) -> Result<Json<CreateTruckResponse>, HttpError> {
    // Only Admin or Manager can create a truck
    if jwt_auth.user.role != crate::models::UserRole::Manager 
        && jwt_auth.user.role != crate::models::UserRole::Admin {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    // Create truck
    let truck = app_state.db_client
        .create_truck(&body.trucknumber, &body.model, body.max_allowance)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(CreateTruckResponse {
        truckid: truck.truckid,
        message: "Truck created successfully".to_string(),
    }))
}

pub async fn get_all_trucks(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<CreateTruckResponse>>, HttpError> {
    // Only Admin or Manager can view all trucks
    if jwt_auth.user.role != crate::models::UserRole::Manager 
        && jwt_auth.user.role != crate::models::UserRole::Admin {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    let trucks = app_state.db_client
        .get_all_trucks()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response: Vec<CreateTruckResponse> = trucks
        .into_iter()
        .map(|t| CreateTruckResponse {
            truckid: t.truckid,
            message: format!("Truck {} ({})", t.trucknumber, t.model),
        })
        .collect();

    Ok(Json(response))
}
