use crate::config::Config;
use crate::utils::error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub username: String,
    pub email: String,
}

pub fn create_token(
    user_id: Uuid,
    username: &str,
    email: &str,
    config: &Config,
) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(config.jwt_expiration))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        username: username.to_string(),
        email: email.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Failed to create token: {}", e)))
}

pub fn verify_token(token: &str, config: &Config) -> Result<TokenData<Claims>, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Auth(format!("Invalid token: {}", e)))
} 