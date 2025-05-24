use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::config::Config;
use crate::db::{transactions, Database};
use crate::middleware::auth::CurrentUser;
use crate::models::transaction::{
    CreateTransactionRequest, TransactionListResponse, TransactionResponse,
};
use crate::utils::error::AppError;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: usize,
    
    #[serde(default = "default_page_size")]
    pub page_size: usize,
}

fn default_page() -> usize {
    1
}

fn default_page_size() -> usize {
    10
}

pub async fn create_transaction(
    current_user: CurrentUser,
    Extension(db): Extension<Database>,
    State(_config): State<Config>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError> {
    // Manually validate the payload
    if let Err(e) = payload.validate() {
        return Err(AppError::Validation(e));
    }

    let mut client = db.pool.get().await?;
    let transaction = transactions::create_transaction(&mut client, current_user.user_id, &payload).await?;

    Ok(Json(TransactionResponse {
        id: transaction.id,
        source_account_id: transaction.source_account_id,
        destination_account_id: transaction.destination_account_id,
        amount: transaction.amount,
        currency: transaction.currency,
        status: transaction.status.to_string(),
        transaction_type: transaction.transaction_type.to_string(),
        description: transaction.description,
        created_at: transaction.created_at,
        updated_at: transaction.updated_at,
    }))
}

pub async fn get_transaction(
    current_user: CurrentUser,
    Extension(db): Extension<Database>,
    State(_config): State<Config>,
    Path(transaction_id): Path<Uuid>,
) -> Result<Json<TransactionResponse>, AppError> {
    let mut client = db.pool.get().await?;
    
    // Check if the user has access to the transaction
    let has_access = transactions::can_user_access_transaction(
        &mut client, 
        current_user.user_id, 
        transaction_id
    ).await?;
    
    if !has_access {
        return Err(AppError::Forbidden(
            "You do not have permission to access this transaction".to_string(),
        ));
    }
    
    let transaction = transactions::get_transaction_by_id(&mut client, transaction_id).await?;

    Ok(Json(TransactionResponse {
        id: transaction.id,
        source_account_id: transaction.source_account_id,
        destination_account_id: transaction.destination_account_id,
        amount: transaction.amount,
        currency: transaction.currency,
        status: transaction.status.to_string(),
        transaction_type: transaction.transaction_type.to_string(),
        description: transaction.description,
        created_at: transaction.created_at,
        updated_at: transaction.updated_at,
    }))
}

pub async fn list_transactions(
    current_user: CurrentUser,
    Extension(db): Extension<Database>,
    State(_config): State<Config>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<TransactionListResponse>, AppError> {
    let mut client = db.pool.get().await?;
    
    let (transactions, total) = transactions::get_user_transactions(
        &mut client, 
        current_user.user_id,
        params.page,
        params.page_size,
    ).await?;

    let transaction_responses = transactions
        .into_iter()
        .map(|transaction| TransactionResponse {
            id: transaction.id,
            source_account_id: transaction.source_account_id,
            destination_account_id: transaction.destination_account_id,
            amount: transaction.amount,
            currency: transaction.currency,
            status: transaction.status.to_string(),
            transaction_type: transaction.transaction_type.to_string(),
            description: transaction.description,
            created_at: transaction.created_at,
            updated_at: transaction.updated_at,
        })
        .collect();

    Ok(Json(TransactionListResponse {
        transactions: transaction_responses,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
} 