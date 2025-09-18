use axum::{Extension, Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState, Error, Result,
    auth::{
        jwt::Claims,
        password::{hash_password, verify_password},
    },
    repository::user_repository::UserRepository,
};

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/register", post(register))
}

async fn login(
    db: Extension<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    payload
        .validate()
        .map_err(|e| Error::BadRequest(format!("Validation error: {}", e)))?;

    let repository = UserRepository::new(&db.pg_pool);

    let user = repository.get_user_by_email(&payload.email).await?;
    let user = user.ok_or(Error::Unauthorized)?;

    let is_valid = verify_password(&payload.password, &user.password)?;

    if !is_valid {
        return Err(Error::Unauthorized);
    }

    let secret = std::env::var("JWT_SECRET").map_err(|_| {
        tracing::error!("JWT_SECRET not set in environment");
        Error::InternalServerError
    })?;

    let token =
        crate::auth::jwt::create_jwt(user.id, &secret).map_err(|_| Error::InternalServerError)?;

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".into(),
    }))
}

async fn register(
    db: Extension<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<&'static str>> {
    payload
        .validate()
        .map_err(|e| Error::BadRequest(format!("Validation error: {}", e)))?;

    let repository = UserRepository::new(&db.pg_pool);

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

#[derive(Debug, Deserialize, Validate)]
struct LoginRequest {
    #[validate(email, length(min = 3, max = 255))]
    email: String,
    #[validate(length(min = 8, max = 128))]
    password: String,
}

#[derive(Debug, Deserialize, Validate)]
struct RegisterRequest {
    #[validate(email, length(min = 3, max = 255))]
    email: String,
    #[validate(length(min = 8, max = 128))]
    password: String,
}
