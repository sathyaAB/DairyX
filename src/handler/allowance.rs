use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::dtos::{CreateAllowanceRequest, CreateAllowanceResponse, CreateTruckAllowanceRequest, CreateTruckAllowanceResponse};
use crate::error::{HttpError, ErrorMessage};
use crate::db::AllowanceExt;
use crate::middleware::JWTAuthMiddeware;
use crate::AppState;
use axum::routing::post;
use axum::Router;

pub fn allowance_handler() -> Router {
    Router::new()
        .route("/create", post(create_allowance))
        .route("/truck-create", post(create_truck_allowance))
}

pub async fn create_allowance(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateAllowanceRequest>,
) -> Result<Json<CreateAllowanceResponse>, HttpError> {
    // Only Manager or Admin can create allowance
    if jwt_auth.user.role != crate::models::UserRole::Manager &&
       jwt_auth.user.role != crate::models::UserRole::Admin {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    let allowance = app_state.db_client
        .create_allowance(body.date, body.amount, body.notes.clone())
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(CreateAllowanceResponse {
        allowanceid: allowance.allowanceid,
        message: "Allowance recorded successfully".to_string(),
    }))
}

pub async fn create_truck_allowance(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateTruckAllowanceRequest>,
) -> Result<Json<CreateTruckAllowanceResponse>, HttpError> {

    if jwt_auth.user.role != crate::models::UserRole::Manager &&
       jwt_auth.user.role != crate::models::UserRole::Admin {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }
    let truck_allowance = app_state.db_client
        .create_truck_allowance(body.allowanceid, body.truckid, body.amount)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(CreateTruckAllowanceResponse {
        truck_allowance_id: truck_allowance.id,
        message: "Truck allowance created successfully".to_string(),
    }))
}