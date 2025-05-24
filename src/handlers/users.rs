use axum::{
    extract::{Extension, State},
    Json,
};
use validator::Validate;

use crate::config::Config;
use crate::db::{users, Database};
use crate::middleware::auth::CurrentUser;
use crate::models::user::{UpdateUserRequest, UserResponse};
use crate::utils::error::AppError;

pub async fn get_profile(
    current_user: CurrentUser,
    Extension(db): Extension<Database>,
    State(_config): State<Config>,
) -> Result<Json<UserResponse>, AppError> {
    let client = db.pool.get().await?;
    let user = users::get_user_by_id(&client, current_user.user_id).await?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        full_name: user.full_name,
        created_at: user.created_at,
    }))
}

pub async fn update_profile(
    current_user: CurrentUser,
    Extension(db): Extension<Database>,
    State(_config): State<Config>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    // Validate the payload
    payload.validate()?;

    let client = db.pool.get().await?;
    let user = users::update_user(&client, current_user.user_id, &payload).await?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        full_name: user.full_name,
        created_at: user.created_at,
    }))
} 