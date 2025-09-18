use axum::{
    Extension, Json, Router,
    routing::{get, post},
};
use bigdecimal::{BigDecimal, FromPrimitive};
use serde::Deserialize;
use validator::Validate;

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
    let user = user.ok_or(crate::Error::Unauthorized)?;

    let balance = user
        .balance
        .to_plain_string()
        .parse::<f64>()
        .map_err(|_| crate::Error::InternalServerError)?;

    Ok(Json(balance))
}

async fn deposit(
    claims: Claims,
    db: Extension<AppState>,
    Json(payload): Json<DepositRequest>,
) -> Result<Json<&'static str>> {
    payload
        .validate()
        .map_err(|e| crate::Error::BadRequest(format!("Validation error: {}", e)))?;

    let repository = UserRepository::new(&db.pool);

    let user = repository.get_user_by_id(claims.user_id).await?;
    let user = user.ok_or(crate::Error::Unauthorized)?;

    let amount_bd = BigDecimal::from_f64(payload.amount)
        .ok_or_else(|| crate::Error::BadRequest("Invalid amount format".into()))?;
    let new_balance = user.balance + amount_bd;
    repository.update_user_balance(user.id, new_balance).await?;

    Ok(Json("Deposit successful"))
}

async fn withdraw(
    claims: Claims,
    db: Extension<AppState>,
    Json(payload): Json<WithdrawRequest>,
) -> Result<Json<&'static str>> {
    payload
        .validate()
        .map_err(|e| crate::Error::BadRequest(format!("Validation error: {}", e)))?;

    let repository = UserRepository::new(&db.pool);
    let user = repository.get_user_by_id(claims.user_id).await?;
    let user = user.ok_or(crate::Error::Unauthorized)?;

    let amount_bd = BigDecimal::from_f64(payload.amount)
        .ok_or_else(|| crate::Error::BadRequest("Invalid amount format".into()))?;
    let new_balance = user.balance - amount_bd;
    if new_balance < BigDecimal::from(0) {
        return Err(crate::Error::BadRequest("Insufficient funds".into()));
    }
    repository.update_user_balance(user.id, new_balance).await?;

    Ok(Json("Withdraw successful"))
}

#[derive(Debug, Deserialize, Validate)]
struct DepositRequest {
    #[validate(range(min = 0.01, max = 1_000_000.0))]
    amount: f64,
}

#[derive(Debug, Deserialize, Validate)]
struct WithdrawRequest {
    #[validate(range(min = 0.01, max = 1_000_000.0))]
    amount: f64,
}
