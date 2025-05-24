use axum::{
    routing::{get, put},
    Router,
};
use crate::handlers::users::{get_profile, update_profile};
use crate::config::Config;

pub fn create_router() -> Router<Config> {
    Router::new()
        .route("/me", get(get_profile))
        .route("/me", put(update_profile))
} 