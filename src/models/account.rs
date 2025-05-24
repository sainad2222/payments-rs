use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: i64,
    pub currency: String,
    pub status: AccountStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Suspended,
    Closed,
}

impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountStatus::Active => write!(f, "active"),
            AccountStatus::Suspended => write!(f, "suspended"),
            AccountStatus::Closed => write!(f, "closed"),
        }
    }
}

impl From<&str> for AccountStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" => AccountStatus::Active,
            "suspended" => AccountStatus::Suspended,
            "closed" => AccountStatus::Closed,
            _ => AccountStatus::Active,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAccountRequest {
    #[validate(length(equal = 3, message = "Currency code must be 3 characters"))]
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: i64,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountListResponse {
    pub accounts: Vec<AccountResponse>,
} 