use axum::{
    extract::{Path, Extension},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::dtos::{CreateProductDto, ProductResponseDto, ProductsListResponseDto};
use crate::error::{HttpError, ErrorMessage};
use crate::db::{ ProductExt};
use crate::models::UserRole;
use crate::middleware::JWTAuthMiddeware;
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use uuid::Uuid;

pub fn products_handler() -> Router {
    Router::new()
        // Create a new product
        .route("/create", post(create_product))
        
        // Get a single product by ID
        .route("/:id", get(get_product))

        // Get all products
        .route("/all", get(get_all_products))

}

pub async fn create_product(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateProductDto>,
) -> Result<Json<ProductResponseDto>, HttpError> {
    // Check if user has Manager or Admin role
    if jwt_auth.user.role != UserRole::Manager && jwt_auth.user.role != UserRole::Admin {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    // Create product
    let product = app_state.db_client
        .create_product(&body.name, body.price, &body.unit_type, body.commission)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(ProductResponseDto {
        status: "success".to_string(),
        product,
    }))
}

pub async fn get_product(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(product_id): Path<String>,
) -> Result<Json<ProductResponseDto>, HttpError> {
    let product_uuid = Uuid::parse_str(&product_id)
        .map_err(|_| HttpError::bad_request("Invalid product ID".to_string()))?;

    let product = app_state.db_client
        .get_product_by_id(product_uuid)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or(HttpError::bad_request("Product not found".to_string()))?;

    Ok(Json(ProductResponseDto {
        status: "success".to_string(),
        product,
    }))
}

pub async fn get_all_products(
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<Json<ProductsListResponseDto >, HttpError> {
    let products = app_state.db_client
        .get_all_products()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(ProductsListResponseDto {
        status: "success".to_string(),
        results: products.len() as usize,
        products,
    }))
}



