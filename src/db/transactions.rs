use crate::db::accounts;
use crate::models::transaction::{
    CreateTransactionRequest, Transaction, TransactionStatus, TransactionType,
};
use crate::utils::error::AppError;
use deadpool_postgres::Client;
use serde_json::json;
use uuid::Uuid;
use crate::db::decimal::PgDecimal;
use rust_decimal::Decimal;

pub async fn create_transaction(
    client: &mut Client,
    user_id: Uuid,
    data: &CreateTransactionRequest,
) -> Result<Transaction, AppError> {
    // Begin a transaction
    let tx = client.transaction().await?;

    // Parse transaction type
    let transaction_type = TransactionType::from(data.transaction_type.as_str());

    // Validate the request based on transaction type
    match transaction_type {
        TransactionType::Deposit => {
            if data.destination_account_id.is_none() {
                return Err(AppError::BadRequest(
                    "Destination account is required for deposits".to_string(),
                ));
            }

            // Verify the destination account belongs to the user
            let dest_account_id = data.destination_account_id.unwrap();
            let dest_account = accounts::get_account(&tx, dest_account_id).await?;
            if dest_account.user_id != user_id {
                return Err(AppError::Forbidden(
                    "You do not have permission to deposit to this account".to_string(),
                ));
            }

            // Ensure currency matches the account
            if dest_account.currency != data.currency {
                return Err(AppError::BadRequest(format!(
                    "Currency mismatch: transaction is in {}, but account is in {}",
                    data.currency, dest_account.currency
                )));
            }
        }
        TransactionType::Withdrawal => {
            if data.source_account_id.is_none() {
                return Err(AppError::BadRequest(
                    "Source account is required for withdrawals".to_string(),
                ));
            }

            // Verify the source account belongs to the user
            let source_account_id = data.source_account_id.unwrap();
            let source_account = accounts::get_account(&tx, source_account_id).await?;
            if source_account.user_id != user_id {
                return Err(AppError::Forbidden(
                    "You do not have permission to withdraw from this account".to_string(),
                ));
            }

            // Ensure currency matches the account
            if source_account.currency != data.currency {
                return Err(AppError::BadRequest(format!(
                    "Currency mismatch: transaction is in {}, but account is in {}",
                    data.currency, source_account.currency
                )));
            }

            // Check if sufficient funds
            if source_account.balance < data.amount {
                return Err(AppError::BadRequest(format!(
                    "Insufficient funds: balance is {}, but withdrawal amount is {}",
                    source_account.balance, data.amount
                )));
            }
        }
        TransactionType::Transfer => {
            if data.source_account_id.is_none() || data.destination_account_id.is_none() {
                return Err(AppError::BadRequest(
                    "Both source and destination accounts are required for transfers".to_string(),
                ));
            }

            // Verify both accounts belongs to the user
            let source_account_id = data.source_account_id.unwrap();
            let dest_account_id = data.destination_account_id.unwrap();

            let source_account = accounts::get_account(&tx, source_account_id).await?;
            if source_account.user_id != user_id {
                return Err(AppError::Forbidden(
                    "You do not have permission to transfer from this account".to_string(),
                ));
            }

            let dest_account = accounts::get_account(&tx, dest_account_id).await?;

            // Ensure currency matches the accounts
            if source_account.currency != data.currency {
                return Err(AppError::BadRequest(format!(
                    "Currency mismatch: transaction is in {}, but source account is in {}",
                    data.currency, source_account.currency
                )));
            }

            if dest_account.currency != data.currency {
                return Err(AppError::BadRequest(format!(
                    "Currency mismatch: transaction is in {}, but destination account is in {}",
                    data.currency, dest_account.currency
                )));
            }

            // Check if sufficient funds
            if source_account.balance < data.amount {
                return Err(AppError::BadRequest(format!(
                    "Insufficient funds: balance is {}, but transfer amount is {}",
                    source_account.balance, data.amount
                )));
            }
        }
    }

    // Create the transaction record
    let row = tx
        .query_one(
            "INSERT INTO transactions 
             (source_account_id, destination_account_id, amount, currency, status, transaction_type, description) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) 
             RETURNING id, source_account_id, destination_account_id, amount, currency, status, 
                     transaction_type, description, created_at, updated_at",
            &[
                &data.source_account_id,
                &data.destination_account_id,
                &PgDecimal(data.amount),
                &data.currency,
                &"pending",
                &data.transaction_type,
                &data.description,
            ],
        )
        .await?;

    let transaction_id: Uuid = row.get("id");

    // Add transaction event
    tx.execute(
        "INSERT INTO transaction_events (transaction_id, previous_status, new_status, event_data) 
         VALUES ($1, NULL, $2, $3)",
        &[
            &transaction_id,
            &"pending",
            &json!({"user_id": user_id.to_string(), "action": "created"}),
        ],
    )
    .await?;

    // Process the transaction
    match transaction_type {
        TransactionType::Deposit => {
            let dest_account_id = data.destination_account_id.unwrap();
            accounts::update_balance(&tx, dest_account_id, data.amount).await?;

            // Update transaction status to completed
            tx.execute(
                "UPDATE transactions SET status = $1, updated_at = NOW() WHERE id = $2",
                &[&"completed", &transaction_id],
            )
            .await?;

            // Add transaction event
            tx.execute(
                "INSERT INTO transaction_events (transaction_id, previous_status, new_status, event_data) 
                 VALUES ($1, $2, $3, $4)",
                &[
                    &transaction_id,
                    &"pending",
                    &"completed",
                    &json!({"user_id": user_id.to_string(), "action": "processed"}),
                ],
            )
            .await?;
        }
        TransactionType::Withdrawal => {
            let source_account_id = data.source_account_id.unwrap();
            // Subtract the amount (negative value)
            accounts::update_balance(&tx, source_account_id, -data.amount).await?;

            // Update transaction status to completed
            tx.execute(
                "UPDATE transactions SET status = $1, updated_at = NOW() WHERE id = $2",
                &[&"completed", &transaction_id],
            )
            .await?;

            // Add transaction event
            tx.execute(
                "INSERT INTO transaction_events (transaction_id, previous_status, new_status, event_data) 
                 VALUES ($1, $2, $3, $4)",
                &[
                    &transaction_id,
                    &"pending",
                    &"completed",
                    &json!({"user_id": user_id.to_string(), "action": "processed"}),
                ],
            )
            .await?;
        }
        TransactionType::Transfer => {
            let source_account_id = data.source_account_id.unwrap();
            let dest_account_id = data.destination_account_id.unwrap();

            // Subtract from source account
            accounts::update_balance(&tx, source_account_id, -data.amount).await?;
            // Add to destination account
            accounts::update_balance(&tx, dest_account_id, data.amount).await?;

            // Update transaction status to completed
            tx.execute(
                "UPDATE transactions SET status = $1, updated_at = NOW() WHERE id = $2",
                &[&"completed", &transaction_id],
            )
            .await?;

            // Add transaction event
            tx.execute(
                "INSERT INTO transaction_events (transaction_id, previous_status, new_status, event_data) 
                 VALUES ($1, $2, $3, $4)",
                &[
                    &transaction_id,
                    &"pending",
                    &"completed",
                    &json!({"user_id": user_id.to_string(), "action": "processed"}),
                ],
            )
            .await?;
        }
    }

    // Commit the transaction
    tx.commit().await?;

    // Get the updated transaction
    get_transaction_by_id(client, transaction_id).await
}

pub async fn get_transaction_by_id(
    client: &Client,
    transaction_id: Uuid,
) -> Result<Transaction, AppError> {
    let row = client
        .query_opt(
            "SELECT id, source_account_id, destination_account_id, amount, 
                    currency, status, transaction_type, description, created_at, updated_at 
             FROM transactions 
             WHERE id = $1",
            &[&transaction_id],
        )
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!("Transaction not found: {}", transaction_id))
        })?;

    Ok(Transaction {
        id: row.get("id"),
        source_account_id: row.get("source_account_id"),
        destination_account_id: row.get("destination_account_id"),
        amount: Decimal::from(row.get::<_, PgDecimal>("amount")),
        currency: row.get("currency"),
        status: TransactionStatus::from(row.get::<_, &str>("status")),
        transaction_type: TransactionType::from(row.get::<_, &str>("transaction_type")),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn get_user_transactions(
    client: &Client,
    user_id: Uuid,
    page: usize,
    page_size: usize,
) -> Result<(Vec<Transaction>, usize), AppError> {
    // Get the total count
    let total_row = client
        .query_one(
            "SELECT COUNT(*) as total
             FROM transactions t
             JOIN accounts a 
                ON t.source_account_id = a.id OR t.destination_account_id = a.id
             WHERE a.user_id = $1",
            &[&user_id],
        )
        .await?;

    let total: i64 = total_row.get("total");
    
    // Skip for pagination
    let offset = (page - 1) * page_size;

    // Get the transactions
    let rows = client
        .query(
            "SELECT t.id, t.source_account_id, t.destination_account_id, t.amount, 
                    t.currency, t.status, t.transaction_type, t.description, t.created_at, t.updated_at 
             FROM transactions t
             JOIN accounts a 
                ON t.source_account_id = a.id OR t.destination_account_id = a.id
             WHERE a.user_id = $1
             ORDER BY t.created_at DESC
             LIMIT $2 OFFSET $3",
            &[&user_id, &(page_size as i64), &(offset as i64)],
        )
        .await?;

    let transactions = rows
        .iter()
        .map(|row| Transaction {
            id: row.get("id"),
            source_account_id: row.get("source_account_id"),
            destination_account_id: row.get("destination_account_id"),
            amount: Decimal::from(row.get::<_, PgDecimal>("amount")),
            currency: row.get("currency"),
            status: TransactionStatus::from(row.get::<_, &str>("status")),
            transaction_type: TransactionType::from(row.get::<_, &str>("transaction_type")),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok((transactions, total as usize))
}

pub async fn can_user_access_transaction(
    client: &Client,
    user_id: Uuid,
    transaction_id: Uuid,
) -> Result<bool, AppError> {
    let row = client
        .query_opt(
            "SELECT 1
             FROM transactions t
             JOIN accounts a 
                ON t.source_account_id = a.id OR t.destination_account_id = a.id
             WHERE a.user_id = $1 AND t.id = $2",
            &[&user_id, &transaction_id],
        )
        .await?;

    Ok(row.is_some())
} 