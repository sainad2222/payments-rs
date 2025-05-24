use crate::{
    config::Config,
    handlers::users::{get_profile, update_profile},
};
use axum::{
    Router,
    routing::{get, put},
};

pub fn create_router() -> Router<Config> {
    Router::new()
        .route("/me", get(get_profile))
        .route("/me", put(update_profile))
}

