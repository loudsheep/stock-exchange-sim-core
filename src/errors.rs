use axum::response::IntoResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Database(sqlx::Error),
    NotFound,
    Unauthorized,
    BadRequest(String),
    InternalServerError,
    LoginFailed,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            Error::Database(e) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
            }
            Error::NotFound => (axum::http::StatusCode::NOT_FOUND, "Resource not found".to_string()),
            Error::Unauthorized => (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            Error::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, format!("Bad request: {}", msg)),
            Error::InternalServerError => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            Error::LoginFailed => (axum::http::StatusCode::UNAUTHORIZED, "Login failed".to_string()),
        };

        // let body = axum::Json(serde_json::json!({
        //     "error": error_message,
        // }));

        (status, error_message).into_response()
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
        }
    }
}

impl std::error::Error for Error {}
