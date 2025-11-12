use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use crate::dtos::{CreatePaymentRequest, CreatePaymentResponse};
use crate::error::{HttpError, ErrorMessage};
use crate::db::{DBClient, PaymentExt};
use crate::middleware::JWTAuthMiddeware;
use crate::AppState;
use axum::routing::post;
use axum::Router;

pub fn payment_handler() -> Router {
    Router::new()
        .route("/create", post(create_payment))
}

pub async fn create_payment(
    Extension(jwt_auth): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreatePaymentRequest>,
) -> Result<Json<CreatePaymentResponse>, HttpError> {
    // Only Driver can make a payment
    if jwt_auth.user.role != crate::models::UserRole::Driver {
        return Err(HttpError::new(
            ErrorMessage::PermissionDenied.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    // Create payment
    let payment = app_state.db_client
        .create_payment(body.salesid, body.amount, body.method.clone(), body.date)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(CreatePaymentResponse {
        paymentid: payment.paymentid,
        message: "Payment created successfully".to_string(),
    }))
}
