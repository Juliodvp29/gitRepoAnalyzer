use std::sync::Arc;
use axum::Router;
use tower_http::cors::CorsLayer;

mod routes;
mod models;
mod services;

use routes::{AppState, create_router};
use services::ai::AiService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY no encontrada en .env");

    let state = Arc::new(AppState {
        ai_service: AiService::new(api_key),
    });

    let app = Router::new()
        .merge(create_router(state))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::info!("Servidor corriendo en http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}