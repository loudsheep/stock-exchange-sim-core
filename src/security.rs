//! Security middleware for adding security headers and other protections

use axum::{
    http::{HeaderValue, Request, Response},
    middleware::Next,
};
use std::time::Duration;

/// Add security headers to all responses
pub async fn security_headers<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response<axum::body::Body> 
where
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: std::error::Error + Send + Sync,
{
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    
    // Prevent clickjacking attacks
    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY"),
    );
    
    // Prevent MIME type sniffing
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    
    // Enable XSS protection
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );
    
    // Strict transport security for HTTPS (if enabled)
    headers.insert(
        "Strict-Transport-Security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    
    // Content Security Policy for additional protection
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_static("default-src 'self'"),
    );
    
    // Referrer policy
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    
    // Permissions policy
    headers.insert(
        "Permissions-Policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );

    response
}

/// Request timeout middleware to prevent slowloris attacks
pub async fn request_timeout<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response<axum::body::Body>, axum::http::StatusCode>
where
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: std::error::Error + Send + Sync,
{
    let timeout_duration = Duration::from_secs(30); // 30 second timeout
    
    match tokio::time::timeout(timeout_duration, next.run(request)).await {
        Ok(response) => Ok(response),
        Err(_) => {
            tracing::warn!("Request timed out after {:?}", timeout_duration);
            Err(axum::http::StatusCode::REQUEST_TIMEOUT)
        }
    }
}

/// Security utilities and helper functions
pub mod utils {
    /// Validate ticker format for security
    pub fn is_valid_ticker_format(ticker: &str) -> bool {
        if ticker.is_empty() || ticker.len() > 10 {
            return false;
        }
        ticker.chars().all(|c| c.is_ascii_alphanumeric())
    }
    
    /// Sanitize string input to prevent injection attacks
    pub fn sanitize_string_input(input: &str, max_length: usize) -> String {
        input
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .take(max_length)
            .collect()
    }
    
    /// Validate email format securely
    pub fn is_valid_email(email: &str) -> bool {
        // Basic email validation - in production use a proper email validation library
        email.contains('@') && email.len() <= 254 && !email.starts_with('@') && !email.ends_with('@')
    }
}