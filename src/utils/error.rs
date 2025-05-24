use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authorization error: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[allow(dead_code)]
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Auth(message) => (StatusCode::UNAUTHORIZED, message),
            AppError::Forbidden(message) => (StatusCode::FORBIDDEN, message),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::Validation(errors) => {
                let validation_errors = errors
                    .field_errors()
                    .iter()
                    .map(|(field, errors)| {
                        (
                            field.to_string(),
                            errors
                                .iter()
                                .map(|error| error.message.as_ref().unwrap_or(&error.code).to_string())
                                .collect::<Vec<String>>(),
                        )
                    })
                    .collect::<std::collections::HashMap<_, _>>();
                (
                    StatusCode::BAD_REQUEST,
                    format!("Validation error: {:?}", validation_errors),
                )
            }
            AppError::Database(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded".to_string(),
            ),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(error: deadpool_postgres::PoolError) -> Self {
        AppError::Database(format!("Database pool error: {}", error))
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(error: tokio_postgres::Error) -> Self {
        AppError::Database(format!("Database error: {}", error))
    }
} 