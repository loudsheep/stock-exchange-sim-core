use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};

use crate::{auth::jwt::Claims, Error, Result};

pub fn routes() -> Router {
    Router::new()
    .route("/login", post(login))
    .route("/logout", post(logout))
}

async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<AuthBody>> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(Error::BadRequest(
            "Username and password cannot be empty".into(),
        ));
    }

    // Dummy authentication logic for demonstration purposes
    if payload.username != "user" && payload.password != "password" {
        return Err(Error::Unauthorized);
    }

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = crate::auth::jwt::create_jwt("user_id_123", &secret)
        .map_err(|_| Error::InternalServerError)?;


    Ok(Json(AuthBody {
        access_token: token,
        token_type: "Bearer".into(),
    }))
}

async fn logout(claims: Claims) -> Result<Json<&'static str>> {
    // In a real application, you might want to invalidate the token here
    Ok(Json("Logged out successfully"))
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
