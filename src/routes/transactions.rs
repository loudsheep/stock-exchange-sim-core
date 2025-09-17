use axum::{Extension, Json, Router, routing::get};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    AppState, Result,
    auth::jwt::Claims,
    repository::{transaction_repository::TransactionRepository, user_repository::UserRepository},
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_transactions))
        .route("/buy", axum::routing::post(create_buy_transaction))
        // .route("/sell", axum::routing::post(create_sell_transaction))
}

async fn get_transactions(
    claims: Claims,
    db: Extension<AppState>,
) -> Result<Json<Vec<TransactionResponse>>> {
    let users_repository = UserRepository::new(&db.pool);
    let transactions_repository = TransactionRepository::new(&db.pool);

    let user = users_repository.get_user_by_id(claims.user_id).await?;
    if user.is_none() {
        return Err(crate::Error::Unauthorized);
    }
    let user = user.unwrap();

    let transactions = transactions_repository
        .get_transactions_by_user(user.id)
        .await?;

    let response: Vec<TransactionResponse> = transactions
        .into_iter()
        .map(|tx| TransactionResponse {
            id: tx.id,
            ticker: tx.ticker,
            quantity: tx.quantity,
            price: tx.price,
            transaction_type: tx.transaction_type,
        })
        .collect();

    Ok(Json(response))
}

async fn create_buy_transaction(
    claims: Claims,
    db: Extension<AppState>,
    Json(payload): Json<CreateBuyTransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    let users_repository = UserRepository::new(&db.pool);
    let transactions_repository = TransactionRepository::new(&db.pool);

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

    let user_balance_f64 = user.balance.to_string().parse::<f64>().unwrap_or(0.0);
    if (payload.quantity as f64) * payload.price > user_balance_f64 {
        return Err(crate::Error::BadRequest(
            "Insufficient balance for this transaction".into(),
        ));
    }

    let price_bd = BigDecimal::from_str(&payload.price.to_string())
        .map_err(|_| crate::Error::BadRequest("Invalid price format".into()))?;
    let transaction = transactions_repository
        .create_transaction(user.id, &payload.ticker, payload.quantity, price_bd, "buy")
        .await?;

    let response = TransactionResponse {
        id: transaction.id,
        ticker: transaction.ticker,
        quantity: transaction.quantity,
        price: transaction.price,
        transaction_type: transaction.transaction_type,
    };

    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
struct CreateBuyTransactionRequest {
    ticker: String,
    quantity: i32,
    price: f64,
}

#[derive(Debug, Deserialize)]
struct CreateSellTransactionRequest {
    ticker: String,
    quantity: i32,
    price: f64,
}

#[derive(Debug, Serialize)]
struct TransactionResponse {
    id: i32,
    ticker: String,
    quantity: i32,
    price: BigDecimal,
    transaction_type: String,
}
