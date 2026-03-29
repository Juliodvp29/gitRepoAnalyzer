use axum::Router;
use tower_http::cors::CorsLayer;

mod routes;
mod models;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(routes::create_router())
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::info!("Servidor corriendo en http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}