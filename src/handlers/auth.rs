use axum::{
    extract::{Extension, State},
    Json,
};
use validator::Validate;

use crate::config::Config;
use crate::db::{users, Database};
use crate::models::user::{CreateUserRequest, LoginRequest, LoginResponse, UserResponse};
use crate::utils::error::AppError;
use crate::utils::jwt::create_token;

pub async fn register(
    Extension(db): Extension<Database>,
    State(config): State<Config>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    // Validate the payload
    payload.validate()?;

    // Create user in the database
    let client = db.pool.get().await?;
    let user = users::create_user(&client, &payload).await?;

    // Return the user without sensitive information
    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        full_name: user.full_name,
        created_at: user.created_at,
    }))
}

pub async fn login(
    Extension(db): Extension<Database>,
    State(config): State<Config>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Validate the credentials
    let client = db.pool.get().await?;
    let user = users::authenticate_user(&client, &payload.username_or_email, &payload.password).await?;

    // Generate a JWT token
    let token = create_token(user.id, &user.username, &user.email, &config)?;

    // Return the token and user
    Ok(Json(LoginResponse { token, user }))
} 