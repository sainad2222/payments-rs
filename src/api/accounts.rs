use axum::{
    routing::{get, post},
    Router,
};
use crate::handlers::accounts::{create_account, get_account, list_accounts};
use crate::config::Config;

pub fn create_router() -> Router<Config> {
    Router::new()
        .route("/", post(create_account))
        .route("/", get(list_accounts))
        .route("/:id", get(get_account))
} 