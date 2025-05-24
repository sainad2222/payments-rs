use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub source_account_id: Option<Uuid>,
    pub destination_account_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub status: TransactionStatus,
    pub transaction_type: TransactionType,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "pending"),
            TransactionStatus::Completed => write!(f, "completed"),
            TransactionStatus::Failed => write!(f, "failed"),
            TransactionStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl From<&str> for TransactionStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => TransactionStatus::Pending,
            "completed" => TransactionStatus::Completed,
            "failed" => TransactionStatus::Failed,
            "cancelled" => TransactionStatus::Cancelled,
            _ => TransactionStatus::Pending,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "deposit"),
            TransactionType::Withdrawal => write!(f, "withdrawal"),
            TransactionType::Transfer => write!(f, "transfer"),
        }
    }
}

impl From<&str> for TransactionType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "deposit" => TransactionType::Deposit,
            "withdrawal" => TransactionType::Withdrawal,
            "transfer" => TransactionType::Transfer,
            _ => TransactionType::Transfer,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTransactionRequest {
    pub source_account_id: Option<Uuid>,
    pub destination_account_id: Option<Uuid>,
    
    #[validate(custom = "validate_positive_amount")]
    pub amount: Decimal,
    
    #[validate(length(equal = 3, message = "Currency code must be 3 characters"))]
    pub currency: String,
    
    pub transaction_type: String,
    pub description: Option<String>,
}

fn validate_positive_amount(amount: &Decimal) -> Result<(), validator::ValidationError> {
    if *amount <= Decimal::ZERO {
        let mut error = validator::ValidationError::new("positive_amount");
        error.message = Some(std::borrow::Cow::Owned("Amount must be greater than zero".to_string()));
        return Err(error);
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub source_account_id: Option<Uuid>,
    pub destination_account_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    pub transaction_type: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionListResponse {
    pub transactions: Vec<TransactionResponse>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionEvent {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub previous_status: Option<String>,
    pub new_status: String,
    pub event_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
} 