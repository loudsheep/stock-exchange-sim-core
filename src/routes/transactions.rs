use axum::{Extension, Json, Router, routing::get};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    AppState, Result,
    auth::jwt::Claims,
    repository::{
        holdings_repository::HoldingsRepository, transaction_repository::TransactionRepository,
        user_repository::UserRepository,
    },
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_transactions))
        .route("/buy", axum::routing::post(create_buy_transaction))
        .route("/sell", axum::routing::post(create_sell_transaction))
}

async fn get_transactions(
    claims: Claims,
    db: Extension<AppState>,
) -> Result<Json<Vec<TransactionResponse>>> {
    let users_repository = UserRepository::new(&db.pool);
    let transactions_repository = TransactionRepository::new(&db.pool);

    let user = users_repository.get_user_by_id(claims.user_id).await?;
    let user = user.ok_or(crate::Error::Unauthorized)?;

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
    let holdings_repository = HoldingsRepository::new(&db.pool);

    let user = users_repository.get_user_by_id(claims.user_id).await?;
    let user = user.ok_or(crate::Error::Unauthorized)?;

    if payload.quantity <= 0 || payload.price <= BigDecimal::from(0) {
        return Err(crate::Error::BadRequest(
            "Quantity and price must be positive".into(),
        ));
    }

    let user_balance_bd = user.balance.clone();
    let total_cost = BigDecimal::from(payload.quantity) * &payload.price;
    if total_cost > user_balance_bd {
        return Err(crate::Error::BadRequest(
            "Insufficient balance for this transaction".into(),
        ));
    }

    let price_bd = BigDecimal::from_str(&payload.price.to_string())
        .map_err(|_| crate::Error::BadRequest("Invalid price format".into()))?;
    let transaction = transactions_repository
        .create_transaction(user.id, &payload.ticker, payload.quantity, price_bd, "buy")
        .await?;

    let holding = holdings_repository
        .get_holding_by_user_and_ticker(user.id, &payload.ticker)
        .await?;

    let price_bd = BigDecimal::from_str(&payload.price.to_string())
        .map_err(|_| crate::Error::BadRequest("Invalid price format".into()))?;

    let _new_holding = if let Some(existing_holding) = holding {
        let total_quantity = existing_holding.quantity + payload.quantity;
        let average_price = (existing_holding.average_price * existing_holding.quantity
            + price_bd * payload.quantity)
            / total_quantity;
        holdings_repository
            .update_holding(existing_holding.id, total_quantity, average_price)
            .await?
    } else {
        holdings_repository
            .create_holding(user.id, &payload.ticker, payload.quantity, price_bd)
            .await?
    };

    let response = TransactionResponse {
        id: transaction.id,
        ticker: transaction.ticker,
        quantity: transaction.quantity,
        price: transaction.price,
        transaction_type: transaction.transaction_type,
    };

    Ok(Json(response))
}

async fn create_sell_transaction(
    claims: Claims,
    db: Extension<AppState>,
    Json(payload): Json<CreateSellTransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    let users_repository = UserRepository::new(&db.pool);
    let transactions_repository = TransactionRepository::new(&db.pool);
    let holdings_repository = HoldingsRepository::new(&db.pool);

    let user = users_repository.get_user_by_id(claims.user_id).await?;
    let user = user.ok_or(crate::Error::Unauthorized)?;

    if payload.quantity <= 0 || payload.price <= BigDecimal::from(0) {
        return Err(crate::Error::BadRequest(
            "Quantity and price must be positive".into(),
        ));
    }

    let holding = holdings_repository
        .get_holding_by_user_and_ticker(user.id, &payload.ticker)
        .await?;

    let holding = holding.ok_or_else(|| {
        crate::Error::BadRequest("Insufficient holdings for this transaction".into())
    })?;

    if holding.quantity < payload.quantity {
        return Err(crate::Error::BadRequest(
            "Insufficient holdings for this transaction".into(),
        ));
    }

    let price_bd = BigDecimal::from_str(&payload.price.to_string())
        .map_err(|_| crate::Error::BadRequest("Invalid price format".into()))?;
    let transaction = transactions_repository
        .create_transaction(user.id, &payload.ticker, payload.quantity, price_bd, "sell")
        .await?;

    let new_quantity = holding.quantity - payload.quantity;
    holdings_repository
        .update_holding(holding.id, new_quantity, holding.average_price)
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
    price: BigDecimal,
}

#[derive(Debug, Deserialize)]
struct CreateSellTransactionRequest {
    ticker: String,
    quantity: i32,
    price: BigDecimal,
}

#[derive(Debug, Serialize)]
struct TransactionResponse {
    id: i32,
    ticker: String,
    quantity: i32,
    price: BigDecimal,
    transaction_type: String,
}
