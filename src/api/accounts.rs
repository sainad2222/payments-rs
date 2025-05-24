use crate::{
    config::Config,
    handlers::accounts::{create_account, get_account, list_accounts},
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router() -> Router<Config> {
    Router::new()
        .route("/", post(create_account))
        .route("/", get(list_accounts))
        .route("/{{:id}}", get(get_account))
}
