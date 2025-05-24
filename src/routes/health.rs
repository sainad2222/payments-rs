use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::config::Config;

// ... existing code ... 