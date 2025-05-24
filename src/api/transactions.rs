use axum::{
    routing::{get, post},
    Router,
};
use crate::handlers::transactions::{create_transaction, get_transaction, list_transactions};
use crate::config::Config;

pub fn create_router() -> Router<Config> {
    Router::new()
        .route("/", post(create_transaction))
        .route("/", get(list_transactions))
        .route("/:id", get(get_transaction))
} 