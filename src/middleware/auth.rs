use crate::config::Config;
use crate::utils::error::AppError;
use crate::utils::jwt::{verify_token, Claims};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::TokenData;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    Config: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let bearer = parts.headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Auth("Missing or invalid authorization header".to_string()))?;

        // Extract config from the application state
        let config = Config::from_ref(state);

        // Decode the token
        let token_data = verify_token(bearer, &config)?;

        // Convert the subject to a UUID
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AppError::Auth("Invalid user ID in token".to_string()))?;

        Ok(CurrentUser {
            user_id,
            username: token_data.claims.username,
            email: token_data.claims.email,
        })
    }
}

pub fn get_current_user(token_data: &TokenData<Claims>) -> Result<CurrentUser, AppError> {
    let user_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| AppError::Auth("Invalid user ID in token".to_string()))?;

    Ok(CurrentUser {
        user_id,
        username: token_data.claims.username.clone(),
        email: token_data.claims.email.clone(),
    })
} 