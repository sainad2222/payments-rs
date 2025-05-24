#[cfg(test)]
mod auth_tests {
    use crate::models::user::CreateUserRequest;
    use crate::utils::jwt::{create_token, verify_token};
    use crate::config::Config;
    use uuid::Uuid;

    #[test]
    fn test_jwt_token_creation_and_verification() {
        let config = Config {
            database_url: "dummy".to_string(),
            jwt_secret: "test_secret_key".to_string(),
            jwt_expiration: 3600, // 1 hour
            port: 3000,
        };

        let user_id = Uuid::new_v4();
        let username = "testuser";
        let email = "test@example.com";

        let token = create_token(user_id, username, email, &config).unwrap();
        let token_data = verify_token(&token, &config).unwrap();

        assert_eq!(token_data.claims.sub, user_id.to_string());
        assert_eq!(token_data.claims.username, username);
        assert_eq!(token_data.claims.email, email);
    }

    #[test]
    fn test_create_user_request_validation() {
        use validator::Validate;

        // Valid request
        let valid_request = CreateUserRequest {
            email: "user@example.com".to_string(),
            username: "validuser".to_string(),
            password: "password123".to_string(),
            full_name: Some("Valid User".to_string()),
        };
        assert!(valid_request.validate().is_ok());

        // Invalid email
        let invalid_email = CreateUserRequest {
            email: "invalid_email".to_string(),
            username: "validuser".to_string(),
            password: "password123".to_string(),
            full_name: Some("Valid User".to_string()),
        };
        assert!(invalid_email.validate().is_err());

        // Username too short
        let short_username = CreateUserRequest {
            email: "user@example.com".to_string(),
            username: "ab".to_string(), // Too short
            password: "password123".to_string(),
            full_name: Some("Valid User".to_string()),
        };
        assert!(short_username.validate().is_err());

        // Password too short
        let short_password = CreateUserRequest {
            email: "user@example.com".to_string(),
            username: "validuser".to_string(),
            password: "short".to_string(), // Too short
            full_name: Some("Valid User".to_string()),
        };
        assert!(short_password.validate().is_err());
    }
}

#[cfg(test)]
mod transaction_tests {
    use crate::models::transaction::{TransactionType, TransactionStatus};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_transaction_type_conversion() {
        assert_eq!(TransactionType::from("deposit"), TransactionType::Deposit);
        assert_eq!(TransactionType::from("withdrawal"), TransactionType::Withdrawal);
        assert_eq!(TransactionType::from("transfer"), TransactionType::Transfer);
        assert_eq!(TransactionType::from("DEPOSIT"), TransactionType::Deposit);
        assert_eq!(TransactionType::from("WITHDRAWAL"), TransactionType::Withdrawal);
        assert_eq!(TransactionType::from("TRANSFER"), TransactionType::Transfer);
        // Default for unknown types
        assert_eq!(TransactionType::from("unknown"), TransactionType::Transfer);
    }

    #[test]
    fn test_transaction_status_conversion() {
        assert_eq!(TransactionStatus::from("pending"), TransactionStatus::Pending);
        assert_eq!(TransactionStatus::from("completed"), TransactionStatus::Completed);
        assert_eq!(TransactionStatus::from("failed"), TransactionStatus::Failed);
        assert_eq!(TransactionStatus::from("cancelled"), TransactionStatus::Cancelled);
        assert_eq!(TransactionStatus::from("PENDING"), TransactionStatus::Pending);
        assert_eq!(TransactionStatus::from("COMPLETED"), TransactionStatus::Completed);
        assert_eq!(TransactionStatus::from("FAILED"), TransactionStatus::Failed);
        assert_eq!(TransactionStatus::from("CANCELLED"), TransactionStatus::Cancelled);
        // Default for unknown statuses
        assert_eq!(TransactionStatus::from("unknown"), TransactionStatus::Pending);
    }

    #[test]
    fn test_decimal_operations() {
        let balance = Decimal::from_str("100.00").unwrap();
        let deposit = Decimal::from_str("50.50").unwrap();
        let withdrawal = Decimal::from_str("25.25").unwrap();

        // Test addition
        assert_eq!(balance + deposit, Decimal::from_str("150.50").unwrap());
        
        // Test subtraction
        assert_eq!(balance - withdrawal, Decimal::from_str("74.75").unwrap());
        
        // Test comparison
        assert!(balance > withdrawal);
        assert!(withdrawal < deposit);
    }
} 