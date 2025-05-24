use crate::models::user::{CreateUserRequest, User, UpdateUserRequest};
use crate::utils::error::AppError;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use deadpool_postgres::Client;
use rand::rngs::OsRng;
use uuid::Uuid;

pub async fn create_user(
    client: &Client,
    user_data: &CreateUserRequest,
) -> Result<User, AppError> {
    // Generate a password hash
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user_data.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?
        .to_string();

    // Insert the new user into the database
    let row = client
        .query_one(
            "INSERT INTO users (email, username, password_hash, full_name) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id, email, username, password_hash, full_name, created_at, updated_at",
            &[
                &user_data.email,
                &user_data.username,
                &password_hash,
                &user_data.full_name,
            ],
        )
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                if e.to_string().contains("email") {
                    AppError::BadRequest(format!("Email {} is already taken", user_data.email))
                } else if e.to_string().contains("username") {
                    AppError::BadRequest(format!("Username {} is already taken", user_data.username))
                } else {
                    AppError::Database(format!("Database error: {}", e))
                }
            } else {
                AppError::Database(format!("Database error: {}", e))
            }
        })?;

    Ok(User {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        full_name: row.get("full_name"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn get_user_by_id(client: &Client, user_id: Uuid) -> Result<User, AppError> {
    let row = client
        .query_opt(
            "SELECT id, email, username, password_hash, full_name, created_at, updated_at 
             FROM users 
             WHERE id = $1",
            &[&user_id],
        )
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User not found with ID: {}", user_id)))?;

    Ok(User {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        full_name: row.get("full_name"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn get_user_by_email(client: &Client, email: &str) -> Result<User, AppError> {
    let row = client
        .query_opt(
            "SELECT id, email, username, password_hash, full_name, created_at, updated_at 
             FROM users 
             WHERE email = $1",
            &[&email],
        )
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User not found with email: {}", email)))?;

    Ok(User {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        full_name: row.get("full_name"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn get_user_by_username(client: &Client, username: &str) -> Result<User, AppError> {
    let row = client
        .query_opt(
            "SELECT id, email, username, password_hash, full_name, created_at, updated_at 
             FROM users 
             WHERE username = $1",
            &[&username],
        )
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User not found with username: {}", username)))?;

    Ok(User {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        full_name: row.get("full_name"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn update_user(
    client: &Client,
    user_id: Uuid,
    update_data: &UpdateUserRequest,
) -> Result<User, AppError> {
    let mut query = String::from("UPDATE users SET updated_at = $1");
    let now = Utc::now();
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![&now];
    let mut param_count = 2;

    if let Some(email) = &update_data.email {
        query.push_str(&format!(", email = ${}", param_count));
        params.push(email);
        param_count += 1;
    }

    if let Some(username) = &update_data.username {
        query.push_str(&format!(", username = ${}", param_count));
        params.push(username);
        param_count += 1;
    }

    if let Some(full_name) = &update_data.full_name {
        query.push_str(&format!(", full_name = ${}", param_count));
        params.push(full_name);
        param_count += 1;
    }

    query.push_str(&format!(" WHERE id = ${} RETURNING id, email, username, password_hash, full_name, created_at, updated_at", param_count));
    params.push(&user_id);

    let row = client
        .query_opt(&query, &params[..])
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User not found with ID: {}", user_id)))?;

    Ok(User {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        full_name: row.get("full_name"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn authenticate_user(
    client: &Client,
    username_or_email: &str,
    password: &str,
) -> Result<User, AppError> {
    // Try to get user by username or email
    let row = client
        .query_opt(
            "SELECT id, email, username, password_hash, full_name, created_at, updated_at 
             FROM users 
             WHERE username = $1 OR email = $1",
            &[&username_or_email],
        )
        .await?
        .ok_or_else(|| {
            AppError::Auth(format!(
                "Authentication failed for user: {}",
                username_or_email
            ))
        })?;

    let user = User {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        full_name: row.get("full_name"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };

    // Verify password
    let parsed_hash =
        PasswordHash::new(&user.password_hash).map_err(|e| {
            AppError::Internal(format!("Failed to parse password hash: {}", e))
        })?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Auth("Invalid password".to_string()))?;

    Ok(user)
} 