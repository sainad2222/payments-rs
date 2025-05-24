mod accounts;
mod auth;
mod transactions;
mod users;

use axum::{Router, routing::get};
use crate::config::Config;

pub fn create_router() -> Router<Config> {
    Router::new()
        .nest("/api/auth", auth::create_router())
        .nest("/api/users", users::create_router())
        .nest("/api/accounts", accounts::create_router())
        .nest("/api/transactions", transactions::create_router())
        .route("/api/health", get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}

