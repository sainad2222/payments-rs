use crate::utils::error::AppError;
use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
    body::Body,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RateLimiter {
    #[allow(dead_code)]
    store: Arc<Mutex<HashMap<String, (usize, Instant)>>>,
    #[allow(dead_code)]
    requests_per_minute: usize,
}

impl RateLimiter {
    pub fn new(requests_per_minute: usize) -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
            requests_per_minute,
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(60) // Default to 60 requests per minute
    }
}

#[allow(dead_code)]
pub async fn rate_limiter(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    rate_limiter: RateLimiter,
    request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Get client identifier, prioritize API key if present, otherwise use IP
    let client_id = headers
        .get("x-api-key")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| addr.ip().to_string());

    let is_allowed = {
        let mut store = rate_limiter.store.lock().unwrap();
        let now = Instant::now();

        match store.get_mut(&client_id) {
            Some((count, timestamp)) => {
                // If the window has expired, reset the counter
                if now.duration_since(*timestamp) > Duration::from_secs(60) {
                    *count = 1;
                    *timestamp = now;
                    true
                } else {
                    // Increment the counter if within the window
                    if *count < rate_limiter.requests_per_minute {
                        *count += 1;
                        true
                    } else {
                        false
                    }
                }
            }
            None => {
                // First request from this client
                store.insert(client_id.clone(), (1, now));
                true
            }
        }
    };

    if !is_allowed {
        return Err(AppError::RateLimitExceeded);
    }

    Ok(next.run(request).await)
} 