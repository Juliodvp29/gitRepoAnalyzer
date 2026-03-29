use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

mod health;
pub mod analyze;
pub mod compare;

pub use analyze::AppState;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/analyze", post(analyze::analyze_repo))
        .route("/api/compare", post(compare::compare_repos))
        .with_state(state)
}