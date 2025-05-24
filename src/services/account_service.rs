use crate::db::accounts;
use crate::models::account::{Account, CreateAccountRequest};
use crate::utils::error::AppError;
use deadpool_postgres::Client;
use uuid::Uuid;

#[allow(dead_code)]
pub async fn create_account(
    client: &Client,
    user_id: Uuid,
    data: &CreateAccountRequest,
) -> Result<Account, AppError> {
    // Normalize currency code to uppercase
    let normalized_request = CreateAccountRequest {
        currency: data.currency.to_uppercase(),
    };
    
    // Create the account in the database
    accounts::create_account(client, user_id, &normalized_request).await
}

#[allow(dead_code)]
pub async fn get_user_accounts(client: &Client, user_id: Uuid) -> Result<Vec<Account>, AppError> {
    accounts::get_user_accounts(client, user_id).await
}

#[allow(dead_code)]
pub async fn get_account(client: &Client, account_id: Uuid, user_id: Uuid) -> Result<Account, AppError> {
    let account = accounts::get_account(client, account_id).await?;
    
    // Verify the account belongs to the user
    if account.user_id != user_id {
        return Err(AppError::Forbidden(
            "You do not have permission to access this account".to_string(),
        ));
    }
    
    Ok(account)
} 