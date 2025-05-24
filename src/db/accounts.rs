use crate::models::account::{Account, AccountStatus, CreateAccountRequest};
use crate::utils::error::AppError;
use deadpool_postgres::Client;
use uuid::Uuid;

pub async fn create_account(
    client: &Client,
    user_id: Uuid,
    data: &CreateAccountRequest,
) -> Result<Account, AppError> {
    // Check if the user already has an account with this currency
    let existing = client
        .query_opt(
            "SELECT id FROM accounts WHERE user_id = $1 AND currency = $2",
            &[&user_id, &data.currency],
        )
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest(format!(
            "User already has an account in {} currency",
            data.currency
        )));
    }

    // Create the account
    let row = client
        .query_one(
            "INSERT INTO accounts (user_id, currency) 
             VALUES ($1, $2) 
             RETURNING id, user_id, balance, currency, status, created_at, updated_at",
            &[&user_id, &data.currency],
        )
        .await?;

    Ok(Account {
        id: row.get("id"),
        user_id: row.get("user_id"),
        balance: row.get("balance"),
        currency: row.get("currency"),
        status: AccountStatus::from(row.get::<_, &str>("status")),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn get_account<T>(client: &T, account_id: Uuid) -> Result<Account, AppError>
where
    T: deadpool_postgres::GenericClient + Sync + Send,
{
    let row = client
        .query_opt(
            "SELECT id, user_id, balance, currency, status, created_at, updated_at 
             FROM accounts 
             WHERE id = $1",
            &[&account_id],
        )
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Account not found: {}", account_id)))?;

    Ok(Account {
        id: row.get("id"),
        user_id: row.get("user_id"),
        balance: row.get("balance"),
        currency: row.get("currency"),
        status: AccountStatus::from(row.get::<_, &str>("status")),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn get_user_accounts(client: &Client, user_id: Uuid) -> Result<Vec<Account>, AppError> {
    let rows = client
        .query(
            "SELECT id, user_id, balance, currency, status, created_at, updated_at 
             FROM accounts 
             WHERE user_id = $1
             ORDER BY created_at",
            &[&user_id],
        )
        .await?;

    let accounts = rows
        .iter()
        .map(|row| Account {
            id: row.get("id"),
            user_id: row.get("user_id"),
            balance: row.get("balance"),
            currency: row.get("currency"),
            status: AccountStatus::from(row.get::<_, &str>("status")),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(accounts)
}

pub async fn update_balance<T>(
    client: &T,
    account_id: Uuid,
    amount: i64,
) -> Result<Account, AppError>
where
    T: deadpool_postgres::GenericClient + Sync + Send,
{
    let row = client
        .query_opt(
            "UPDATE accounts 
             SET balance = balance + $1, updated_at = NOW() 
             WHERE id = $2 
             RETURNING id, user_id, balance, currency, status, created_at, updated_at",
            &[&amount, &account_id],
        )
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Account not found: {}", account_id)))?;

    Ok(Account {
        id: row.get("id"),
        user_id: row.get("user_id"),
        balance: row.get("balance"),
        currency: row.get("currency"),
        status: AccountStatus::from(row.get::<_, &str>("status")),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}
