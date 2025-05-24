use crate::config::Config;
use deadpool_postgres::{Config as PgConfig, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub mod users;
pub mod accounts;
pub mod transactions;
pub mod decimal;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn new(config: &Config) -> Self {
        let pg_config = PgConfig {
            host: Some(extract_host(&config.database_url)),
            user: Some(extract_user(&config.database_url)),
            password: Some(extract_password(&config.database_url)),
            dbname: Some(extract_dbname(&config.database_url)),
            port: Some(extract_port(&config.database_url)),
            manager: Some(ManagerConfig {
                recycling_method: RecyclingMethod::Fast,
            }),
            ..Default::default()
        };

        let pool = pg_config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Unable to create database connection pool");

        Self { pool }
    }
}

// Helper functions to extract connection details from DATABASE_URL
fn extract_host(db_url: &str) -> String {
    let parts: Vec<&str> = db_url.split('@').collect();
    if parts.len() < 2 {
        return "localhost".to_string();
    }
    let host_port = parts[1].split(':').collect::<Vec<&str>>();
    host_port[0].to_string()
}

fn extract_user(db_url: &str) -> String {
    let parts: Vec<&str> = db_url.split("://").collect();
    if parts.len() < 2 {
        return "postgres".to_string();
    }
    let user_pass = parts[1].split(':').collect::<Vec<&str>>();
    user_pass[0].to_string()
}

fn extract_password(db_url: &str) -> String {
    let parts: Vec<&str> = db_url.split(':').collect();
    if parts.len() < 3 {
        return "".to_string();
    }
    let pass_host = parts[2].split('@').collect::<Vec<&str>>();
    pass_host[0].to_string()
}

fn extract_port(db_url: &str) -> u16 {
    let parts: Vec<&str> = db_url.split('@').collect();
    if parts.len() < 2 {
        return 5432;
    }
    let host_port = parts[1].split(':').collect::<Vec<&str>>();
    if host_port.len() < 2 {
        return 5432;
    }
    let port_db = host_port[1].split('/').collect::<Vec<&str>>();
    port_db[0].parse::<u16>().unwrap_or(5432)
}

fn extract_dbname(db_url: &str) -> String {
    let parts: Vec<&str> = db_url.split('/').collect();
    if parts.len() < 4 {
        return "postgres".to_string();
    }
    parts[3].to_string()
} 