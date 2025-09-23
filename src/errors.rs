use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;
use chrono;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Database(sqlx::Error),
    NotFound,
    Unauthorized,
    BadRequest(String),
    InternalServerError,
    LoginFailed,
    NotImplemented,
    Conflict(String),
    GrpcError(String),
    RedisError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            Error::Database(_e) => {
                // Log the actual error but don't expose it to users
                tracing::error!("Database error: {}", _e);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            },
            Error::NotFound => (
                axum::http::StatusCode::NOT_FOUND,
                "Resource not found".to_string(),
            ),
            Error::Unauthorized => (
                axum::http::StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
            ),
            Error::BadRequest(msg) => {
                // Sanitize error messages to prevent information disclosure
                let sanitized_msg = if msg.len() > 200 {
                    "Invalid request parameters".to_string()
                } else {
                    msg.clone()
                };
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    format!("Bad request: {}", sanitized_msg),
                )
            },
            Error::InternalServerError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            Error::LoginFailed => (
                axum::http::StatusCode::UNAUTHORIZED,
                "Invalid credentials".to_string(),
            ),
            Error::NotImplemented => (
                axum::http::StatusCode::NOT_IMPLEMENTED,
                "Not Implemented".to_string(),
            ),
            Error::Conflict(msg) => (
                axum::http::StatusCode::CONFLICT,
                format!("Conflict: {}", msg),
            ),
            Error::GrpcError(_msg) => {
                // Log the actual error but provide generic message
                tracing::error!("gRPC error: {}", _msg);
                (
                    StatusCode::BAD_GATEWAY,
                    "External service unavailable".to_string(),
                )
            },
            Error::RedisError(_msg) => {
                // Log the actual error but provide generic message
                tracing::error!("Redis error: {}", _msg);
                (
                    StatusCode::BAD_GATEWAY,
                    "Cache service unavailable".to_string(),
                )
            },
        };

        let body = axum::Json(json!({
            "error": error_message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));

        (status, body).into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Database(e) => write!(f, "Database error: {}", e),
            Error::NotFound => write!(f, "Resource not found"),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Error::InternalServerError => write!(f, "Internal server error"),
            Error::LoginFailed => write!(f, "Login failed"),
            Error::NotImplemented => write!(f, "Not Implemented"),
            Error::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Error::GrpcError(msg) => write!(f, "gRPC error: {}", msg),
            Error::RedisError(msg) => write!(f, "Redis error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
