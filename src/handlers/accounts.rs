use axum::{
    Json,
    extract::{Extension, Path, State},
};
use uuid::Uuid;
use validator::Validate;

use crate::config::Config;
use crate::db::{Database, accounts};
use crate::middleware::auth::CurrentUser;
use crate::models::account::{AccountListResponse, AccountResponse, CreateAccountRequest};
use crate::utils::error::AppError;

pub async fn create_account(
    current_user: CurrentUser,
    Extension(db): Extension<Database>,
    State(_config): State<Config>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, AppError> {
    // Validate the payload
    payload.validate()?;

    let client = db.pool.get().await?;
    let account = accounts::create_account(&client, current_user.user_id, &payload).await?;

    Ok(Json(AccountResponse {
        id: account.id,
        user_id: account.user_id,
        balance: account.balance,
        currency: account.currency,
        status: account.status.to_string(),
        created_at: account.created_at,
        updated_at: account.updated_at,
    }))
}

pub async fn get_account(
    current_user: CurrentUser,
    Extension(db): Extension<Database>,
    State(_config): State<Config>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<AccountResponse>, AppError> {
    let client = db.pool.get().await?;
    let account = accounts::get_account(&client, account_id).await?;

    // Ensure the account belongs to the current user
    if account.user_id != current_user.user_id {
        return Err(AppError::Forbidden(
            "You do not have permission to access this account".to_string(),
        ));
    }

    Ok(Json(AccountResponse {
        id: account.id,
        user_id: account.user_id,
        balance: account.balance,
        currency: account.currency,
        status: account.status.to_string(),
        created_at: account.created_at,
        updated_at: account.updated_at,
    }))
}

pub async fn list_accounts(
    current_user: CurrentUser,
    Extension(db): Extension<Database>,
    State(_config): State<Config>,
) -> Result<Json<AccountListResponse>, AppError> {
    let client = db.pool.get().await?;
    let accounts = accounts::get_user_accounts(&client, current_user.user_id).await?;

    let account_responses = accounts
        .into_iter()
        .map(|account| AccountResponse {
            id: account.id,
            user_id: account.user_id,
            balance: account.balance,
            currency: account.currency,
            status: account.status.to_string(),
            created_at: account.created_at,
            updated_at: account.updated_at,
        })
        .collect();

    Ok(Json(AccountListResponse {
        accounts: account_responses,
    }))
}

