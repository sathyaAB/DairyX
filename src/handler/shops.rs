use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::dtos::{CreateShopRequest,CreateShopResponse};
use crate::error::{HttpError, ErrorMessage};
use crate::db::ShopExt;
use crate::middleware::JWTAuthMiddeware;
use crate::AppState;
use axum::routing::{post, get};
use axum::Router;

pub fn shop_handler() -> Router {
    Router::new()
        .route("/create", post(create_shop))
        .route("/all", get(get_all_shops))
}

pub async fn create_shop(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateShopRequest>,
) -> Result<Json<CreateShopResponse>, HttpError> {
    // Only Admin or Manager
    if jwt_auth.user.role != crate::models::UserRole::Manager
        && jwt_auth.user.role != crate::models::UserRole::Admin
    {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    let shop = app_state.db_client
        .create_shop(&body.name, &body.address, body.city.as_deref(), body.district.as_deref(), body.contact_number.as_deref())
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(CreateShopResponse {
        shopid: shop.shopid,
        message: "Shop created successfully".to_string(),
    }))
}

pub async fn get_all_shops(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<CreateShopResponse>>, HttpError> {
    if jwt_auth.user.role != crate::models::UserRole::Manager
        && jwt_auth.user.role != crate::models::UserRole::Admin
    {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    let shops = app_state.db_client
        .get_all_shops()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response: Vec<CreateShopResponse> = shops
        .into_iter()
        .map(|s| CreateShopResponse {
            shopid: s.shopid,
            message: format!("{} ({})", s.name, s.address),
        })
        .collect();

    Ok(Json(response))
}
