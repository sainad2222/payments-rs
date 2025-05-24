use crate::{
    config::Config,
    handlers::auth::{login, register},
};
use axum::{Router, routing::post};

pub fn create_router() -> Router<Config> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

