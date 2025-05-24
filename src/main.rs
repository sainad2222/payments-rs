mod api;
mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod services;
mod tests;
mod utils;

use std::net::SocketAddr;

use axum::{extract::Extension, http::Method};
use config::Config;
use db::Database;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load configuration
    let config = Config::from_env();

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database connection
    let db = Database::new(&config);

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    // Build application routes
    let app = api::create_router()
        .with_state(config.clone())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(Extension(db));

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
