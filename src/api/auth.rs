use axum::{
    routing::post,
    Router,
};
use crate::handlers::auth::{login, register};
use crate::config::Config;

pub fn create_router() -> Router<Config> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
} 