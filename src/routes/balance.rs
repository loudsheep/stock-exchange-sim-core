use axum::{
    Extension, Json, Router,
    routing::{get, post},
};
use bigdecimal::{BigDecimal, FromPrimitive};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{AppState, Result, auth::jwt::Claims, repository::user_repository::UserRepository};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_balance))
        .route("/deposit", post(deposit))
        .route("/withdraw", post(withdraw))
}

async fn get_balance(claims: Claims, db: Extension<AppState>) -> Result<Json<f64>> {
    let repository = UserRepository::new(&db.pool);
    let user = repository.get_user_by_id(claims.user_id).await?;
    if user.is_none() {
        return Err(crate::Error::Unauthorized);
    }
    let user = user.unwrap();

    Ok(Json(
        user.balance.to_plain_string().parse::<f64>().unwrap_or(0.0),
    ))
}

async fn deposit(
    claims: Claims,
    db: Extension<AppState>,
    Json(payload): Json<DepositRequest>,
) -> Result<Json<&'static str>> {
    let repository = UserRepository::new(&db.pool);

    if payload.amount <= 0.0 {
        return Err(crate::Error::BadRequest(
            "Deposit amount must be positive".into(),
        ));
    }

    let user = repository.get_user_by_id(claims.user_id).await?;
    if user.is_none() {
        return Err(crate::Error::Unauthorized);
    }
    let user = user.unwrap();
    let amount_bd = BigDecimal::from_f64(payload.amount).unwrap_or(0.into());
    let new_balance = user.balance + amount_bd;
    repository.update_user_balance(user.id, new_balance).await?;

    Ok(Json("Deposit successful"))
}

async fn withdraw(
    claims: Claims,
    db: Extension<AppState>,
    Json(payload): Json<WithdrawRequest>,
) -> Result<Json<&'static str>> {
    let repository = UserRepository::new(&db.pool);
    if payload.amount <= 0.0 {
        return Err(crate::Error::BadRequest(
            "Withdraw amount must be positive".into(),
        ));
    }
    let user = repository.get_user_by_id(claims.user_id).await?;
    if user.is_none() {
        return Err(crate::Error::Unauthorized);
    }
    let user = user.unwrap();

    let amount_bd = BigDecimal::from_f64(payload.amount).unwrap_or(0.into());
    let new_balance = user.balance - amount_bd;
    if new_balance < BigDecimal::from(0) {
        return Err(crate::Error::BadRequest("Insufficient funds".into()));
    }
    repository.update_user_balance(user.id, new_balance).await?;

    Ok(Json("Withdraw successful"))
}

#[derive(Debug, Deserialize)]
struct DepositRequest {
    amount: f64,
}

#[derive(Debug, Deserialize)]
struct WithdrawRequest {
    amount: f64,
}
