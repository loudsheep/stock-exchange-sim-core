use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};

use crate::{
    AppState, Error, Result,
    auth::jwt::Claims,
    repository::{user_repository::UserRepository},
};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/register", post(register))
        .with_state(state)
}

async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::BadRequest(
            "Email and password cannot be empty".into(),
        ));
    }

    // Dummy authentication logic for demonstration purposes
    if payload.email != "user" && payload.password != "password" {
        return Err(Error::Unauthorized);
    }

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = crate::auth::jwt::create_jwt("user_id_123", &secret)
        .map_err(|_| Error::InternalServerError)?;

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".into(),
    }))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<&'static str>> {
    let repository = UserRepository::new(state.pool.as_ref());
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::BadRequest(
            "Email and password cannot be empty".into(),
        ));
    }

    let user_exists = repository.get_user_by_email(&payload.email).await?;
    if user_exists.is_some() {
        return Err(Error::Conflict("Email already exists".into()));
    }

    repository
        .create_user(&payload.email, &payload.password)
        .await?;
    Ok(Json("User registered successfully"))
}

async fn logout(claims: Claims) -> Result<Json<&'static str>> {
    // In a real application, you might want to invalidate the token here
    Ok(Json("Logged out successfully"))
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}
