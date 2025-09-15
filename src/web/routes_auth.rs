use crate::{Error, Result};

#[derive(Debug, serde::Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

async fn login(payload: axum::Json<AuthRequest>) -> Result<String> {
    let username = payload.username.trim();
    let password = payload.password.trim();

    if username == "admin" && password == "password" {
        Ok("Login successful".to_string())
    } else {
        Err(Error::LoginFailed)
    }
}   