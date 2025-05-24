use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string())
            .parse::<i64>()
            .expect("JWT_EXPIRATION must be a valid integer");
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid integer");

        Self {
            database_url,
            jwt_secret,
            jwt_expiration,
            port,
        }
    }
} 