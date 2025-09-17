use axum::{Extension, Json, Router, routing::post};
use serde::{Deserialize, Serialize};

use crate::{auth::{jwt::Claims, password::{hash_password, verify_password}}, repository::user_repository::UserRepository, AppState, Error, Result};

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/register", post(register))
}

async fn login(db: Extension<AppState>, Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>> {
    let repository = UserRepository::new(&db.pool);

    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::BadRequest(
            "Email and password cannot be empty".into(),
        ));
    }

    let user = repository.get_user_by_email(&payload.email).await?;
    if user.is_none() {
        return Err(Error::Unauthorized);
    }
    
    let user = user.unwrap();
    let is_valid = verify_password(&payload.password, &user.password)?;

    if !is_valid {
        return Err(Error::Unauthorized);
    }

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = crate::auth::jwt::create_jwt(user.id, &secret)
        .map_err(|_| Error::InternalServerError)?;

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".into(),
    }))
}

async fn register(
    db: Extension<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<&'static str>> {
    let repository = UserRepository::new(&db.pool);

    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::BadRequest(
            "Email and password cannot be empty".into(),
        ));
    }

    let user_exists = repository.get_user_by_email(&payload.email).await?;
    if user_exists.is_some() {
        return Err(Error::Conflict("Email already exists".into()));
    }

    let hashed_password = hash_password(&payload.password)?;

    repository
        .create_user(&payload.email, &hashed_password)
        .await?;
    Ok(Json("User registered successfully"))
}

async fn logout(_claims: Claims) -> Result<Json<&'static str>> {
    // TODO invalidate the token here
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
