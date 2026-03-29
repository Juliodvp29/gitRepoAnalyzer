use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

mod health;
pub mod analyze;

pub use analyze::AppState;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/analyze", post(analyze::analyze_repo))
        .with_state(state)
}