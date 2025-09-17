use axum::{
    Extension, Json, Router,
    routing::{get},
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{types::BigDecimal, PgPool};
use std::str::FromStr;

use crate::{auth::jwt::Claims, repository::{transaction_repository::TransactionRepository, user_repository::UserRepository}, Result};

pub fn routes() -> Router {
    Router::new()
    .route("/", get(get_transactions))
    .route("/", axum::routing::post(create_transaction))
}

async fn get_transactions(
    claims: Claims,
    db: Extension<PgPool>,
) -> Result<Json<Vec<TransactionResponse>>> {
    let users_repository = UserRepository::new(&db);
    let transactions_repository = TransactionRepository::new(&db);

    let user = users_repository.get_user_by_id(claims.user_id).await?;
    if user.is_none() {
        return Err(crate::Error::Unauthorized);
    }
    let user = user.unwrap();

    let transactions = transactions_repository.get_transactions_by_user(user.id).await?;

    let response: Vec<TransactionResponse> = transactions
        .into_iter()
        .map(|tx| TransactionResponse {
            id: tx.id,
            ticker: tx.ticker,
            quantity: tx.quantity,
            price: Decimal::from_str_exact(&tx.price.to_string()).unwrap_or(Decimal::ZERO),
            transaction_type: tx.transaction_type,
        })
        .collect();

    Ok(Json(response))
}


async fn create_transaction(
    claims: Claims,
    db: Extension<PgPool>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    let users_repository = UserRepository::new(&db);
    let transactions_repository = TransactionRepository::new(&db);

    let user = users_repository.get_user_by_id(claims.user_id).await?;
    if user.is_none() {
        return Err(crate::Error::Unauthorized);
    }
    let user = user.unwrap();

    if payload.quantity <= 0 || payload.price <= 0.0 {
        return Err(crate::Error::BadRequest(
            "Quantity and price must be positive".into(),
        ));
    }

    let price_bd = BigDecimal::from_str(&payload.price.to_string()).map_err(|_| crate::Error::BadRequest("Invalid price format".into()))?;
    let transaction = transactions_repository
        .create_transaction(
            user.id,
            &payload.ticker,
            payload.quantity,
            price_bd,
            &payload.transaction_type,
        )
        .await?;

    let response = TransactionResponse {
        id: transaction.id,
        ticker: transaction.ticker,
        quantity: transaction.quantity,
        price: Decimal::from_str_exact(&transaction.price.to_string()).unwrap_or(Decimal::ZERO),
        transaction_type: transaction.transaction_type,
    };

    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
struct CreateTransactionRequest {
    ticker: String,
    quantity: i32,
    price: f64,
    transaction_type: String,
}

#[derive(Debug, Serialize)]
struct TransactionResponse {
    id: i32,
    ticker: String,
    quantity: i32,
    price: Decimal,
    transaction_type: String,
}