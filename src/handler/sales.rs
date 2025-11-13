use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::dtos::{CreateSaleRequest, CreateSaleResponse, DailyProductSaleRequest, DailyProductSaleListResponse, DailySalesRevenueResponse};
use crate::error::{HttpError, ErrorMessage};
use crate::db::{SalesExt};
use crate::models::UserRole;
use crate::middleware::JWTAuthMiddeware;
use crate::AppState;
use axum::routing::{get,post};
use axum::Router;
use uuid::Uuid;

pub fn sales_handler() -> Router {
    Router::new()
        .route("/create", post(create_sale))
        .route("/daily-product-sales", get(get_daily_product_sales))
        .route("/daily-sales-revenue", get(get_daily_sales_revenue))
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

pub async fn get_daily_product_sales(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<DailyProductSaleRequest>,
) -> Result<Json<DailyProductSaleListResponse>, HttpError> {
    let sales = app_state.db_client
        .get_daily_product_sales(body.date)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(DailyProductSaleListResponse {
        status: "success".to_string(),
        results: sales.len(),
        sales,
    }))
}

pub async fn get_daily_sales_revenue(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<DailyProductSaleRequest>, 
) -> Result<Json<DailySalesRevenueResponse>, HttpError> {
    let total_revenue = app_state.db_client
        .get_daily_total_sales_revenue(body.date)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(DailySalesRevenueResponse {
        date: body.date,
        total_revenue,
    }))
}

