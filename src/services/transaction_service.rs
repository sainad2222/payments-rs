use crate::db::transactions;
use crate::models::transaction::{CreateTransactionRequest, Transaction};
use crate::utils::error::AppError;
use deadpool_postgres::Client;
use uuid::Uuid;

#[allow(dead_code)]
pub async fn create_transaction(
    client: &mut Client,
    user_id: Uuid,
    data: &CreateTransactionRequest,
) -> Result<Transaction, AppError> {
    // Normalize currency code to uppercase
    let normalized_request = CreateTransactionRequest {
        source_account_id: data.source_account_id,
        destination_account_id: data.destination_account_id,
        amount: data.amount,
        currency: data.currency.to_uppercase(),
        transaction_type: data.transaction_type.to_lowercase(),
        description: data.description.clone(),
    };

    // Process the transaction in the database
    transactions::create_transaction(client, user_id, &normalized_request).await
}

#[allow(dead_code)]
pub async fn get_transaction(
    client: &mut Client,
    transaction_id: Uuid,
    user_id: Uuid,
) -> Result<Transaction, AppError> {
    // Check if the user has access to the transaction
    let has_access = transactions::can_user_access_transaction(client, user_id, transaction_id).await?;

    if !has_access {
        return Err(AppError::Forbidden(
            "You do not have permission to access this transaction".to_string(),
        ));
    }

    // Get the transaction
    transactions::get_transaction_by_id(client, transaction_id).await
}

#[allow(dead_code)]
pub async fn list_user_transactions(
    client: &mut Client,
    user_id: Uuid,
    page: usize,
    page_size: usize,
) -> Result<(Vec<Transaction>, usize), AppError> {
    transactions::get_user_transactions(client, user_id, page, page_size).await
} 