use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::dtos::{CreateSaleRequest, SaleProductItem, CreateSaleResponse};
use crate::error::{HttpError, ErrorMessage};
use crate::db::{DBClient, SalesExt};
use crate::models::UserRole;
use crate::middleware::JWTAuthMiddeware;
use crate::AppState;
use axum::routing::post;
use axum::Router;
use uuid::Uuid;
use chrono::NaiveDate;

pub fn sales_handler() -> Router {
    Router::new()
        .route("/create", post(create_sale))
}

pub async fn create_sale(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateSaleRequest>,
) -> Result<Json<CreateSaleResponse>, HttpError> {
    // ✅ Only drivers can create sales
    if jwt_auth.user.role != UserRole::Driver {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    // Use date from request or default to today
    let date = body.date;

    // Convert products to Vec<(Uuid, i32)>
    let products: Vec<(Uuid, i32)> = body
        .products
        .into_iter()
        .map(|p| (p.product_id, p.quantity))
        .collect();


    // Create sale in DB
    let sale = app_state.db_client
        .create_sale( body.truckload_id, body.shop_id, date, products)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(CreateSaleResponse {
        salesid: sale.salesid,
        message: "Sale recorded successfully".to_string(),
    }))
}
