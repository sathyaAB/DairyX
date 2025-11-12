use std::sync::Arc;

use axum::{middleware, Extension, Router};
use tower_http::trace::TraceLayer;

use crate::{handler::{auth::auth_handler, users::users_handler}, middleware::auth, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/auth", auth_handler())
        .nest(
            "/users", 
            users_handler()
                .layer(middleware::from_fn(auth))
        )
        .nest(
            "/products", 
            crate::handler::products::products_handler()
                .layer(middleware::from_fn(auth))
        )
        .nest(
            "/delivery", 
            crate::handler::delivery::delivery_handler()
                .layer(middleware::from_fn(auth))
        )
        .nest(
            "/truck-load",
            crate::handler::truck_load::truck_load_handler()
                .layer(middleware::from_fn(auth))
        )
        .nest(
            "/sales",
            crate::handler::sales::sales_handler()
                .layer(middleware::from_fn(auth))
        )
        .nest(
            "/payment",
            crate::handler::payment::payment_handler()
                .layer(middleware::from_fn(auth))
        )
        .nest(
            "/allowance",
            crate::handler::allowance::allowance_handler()
                .layer(middleware::from_fn(auth))
        )
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new().nest("/api", api_route)
}