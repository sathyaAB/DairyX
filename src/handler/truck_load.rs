use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::dtos::{CreateTruckLoadRequest, TruckLoadProductItem, CreateTruckLoadResponse};
use crate::error::{HttpError, ErrorMessage};
use crate::db::{DBClient, TruckLoadExt};
use crate::models::UserRole;
use crate::middleware::JWTAuthMiddeware;
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use uuid::Uuid;
use chrono::NaiveDate;

pub fn truck_load_handler() -> Router {
    Router::new()
        .route("/create", post(create_truck_load))
        .route("/history", get(get_truck_load_history))
}
pub async fn create_truck_load(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateTruckLoadRequest>,
) -> Result<Json<CreateTruckLoadResponse>, HttpError> {
    // Only Manager or Admin can create truck loads
    if jwt_auth.user.role != UserRole::Manager && jwt_auth.user.role != UserRole::Admin {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    // Use date from request
    let date = body.date;

    // Convert products to Vec<(Uuid, i32)>
    let products: Vec<(Uuid, i32)> = body
        .products
        .into_iter()
        .map(|p| (p.product_id, p.quantity))
        .collect();

    // ✅ Use the provided driver ID instead of logged-in user's ID
    let driver_id = body.driver_id;

    // Create truck load in DB
    let truck_load = app_state.db_client
        .create_truck_load(driver_id, body.truck_id, date, products)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(CreateTruckLoadResponse {
        truckloadid: truck_load.truckloadid,
        driver_id: truck_load.userid,
        message: "Truck load created successfully".to_string(),
    }))
}

pub async fn get_truck_load_history(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<CreateTruckLoadResponse>>, HttpError> {
    // Only Admin or Manager can view all truck loads
    if jwt_auth.user.role != UserRole::Manager && jwt_auth.user.role != UserRole::Admin {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    // Fetch all truck loads
    let truck_loads = app_state.db_client
        .get_all_truck_loads()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    // Map DB models to response DTOs
    let response: Vec<CreateTruckLoadResponse> = truck_loads
        .into_iter()
        .map(|t| CreateTruckLoadResponse {
            truckloadid: t.truckloadid,
            driver_id: t.userid,
            message: format!("Truck load on {}", t.date),
        })
        .collect();

    Ok(Json(response))
}

