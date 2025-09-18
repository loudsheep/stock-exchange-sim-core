use axum::{
    Extension, Json, Router,
    routing::{get, post},
};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

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

/// Get all transactions for the authenticated user
///
/// Returns a list of all buy and sell transactions made by the user.
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

/// Create a buy transaction
///
/// Creates a new buy transaction for the authenticated user.
/// This operation:
/// 1. Validates the user has sufficient balance
/// 2. Creates a transaction record
/// 3. Updates the user's balance (deducting the cost)
/// 4. Updates or creates a holding record
///
/// All operations should be atomic to ensure data consistency.
async fn create_buy_transaction(
    claims: Claims,
    db: Extension<AppState>,
    Json(payload): Json<CreateBuyTransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    payload
        .validate()
        .map_err(|e| crate::Error::BadRequest(format!("Validation error: {}", e)))?;

    let users_repository = UserRepository::new(&db.pool);
    let transactions_repository = TransactionRepository::new(&db.pool);
    let holdings_repository = HoldingsRepository::new(&db.pool);

    let user = users_repository.get_user_by_id(claims.user_id).await?;
    let user = user.ok_or(crate::Error::Unauthorized)?;

    if payload.price <= BigDecimal::from(0) {
        return Err(crate::Error::BadRequest("Price must be positive".into()));
    }

    let user_balance_bd = user.balance.clone();
    let total_cost = BigDecimal::from(payload.quantity) * &payload.price;
    if total_cost > user_balance_bd {
        return Err(crate::Error::BadRequest(
            "Insufficient balance for this transaction".into(),
        ));
    }

    // Create transaction record first
    let transaction = transactions_repository
        .create_transaction(
            user.id,
            &payload.ticker,
            payload.quantity,
            payload.price.clone(),
            "buy",
        )
        .await?;

    // Update user balance (deduct the cost)
    let new_balance = user_balance_bd - total_cost;
    users_repository
        .update_user_balance(user.id, new_balance)
        .await?;

    // Update or create holding
    let holding = holdings_repository
        .get_holding_by_user_and_ticker(user.id, &payload.ticker)
        .await?;

    let _new_holding = if let Some(existing_holding) = holding {
        let total_quantity = existing_holding.quantity + payload.quantity;
        let average_price = (existing_holding.average_price * existing_holding.quantity
            + &payload.price * payload.quantity)
            / total_quantity;
        holdings_repository
            .update_holding(existing_holding.id, total_quantity, average_price)
            .await?
    } else {
        holdings_repository
            .create_holding(
                user.id,
                &payload.ticker,
                payload.quantity,
                payload.price.clone(),
            )
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

/// Create a sell transaction
///
/// Creates a new sell transaction for the authenticated user.
/// This operation:
/// 1. Validates the user has sufficient holdings
/// 2. Creates a transaction record
/// 3. Updates the user's balance (adding the proceeds)
/// 4. Updates the holding quantity
///
/// All operations should be atomic to ensure data consistency.
async fn create_sell_transaction(
    claims: Claims,
    db: Extension<AppState>,
    Json(payload): Json<CreateSellTransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    payload
        .validate()
        .map_err(|e| crate::Error::BadRequest(format!("Validation error: {}", e)))?;

    let users_repository = UserRepository::new(&db.pool);
    let transactions_repository = TransactionRepository::new(&db.pool);
    let holdings_repository = HoldingsRepository::new(&db.pool);

    let user = users_repository.get_user_by_id(claims.user_id).await?;
    let user = user.ok_or(crate::Error::Unauthorized)?;

    if payload.price <= BigDecimal::from(0) {
        return Err(crate::Error::BadRequest("Price must be positive".into()));
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

    // Create transaction record first
    let transaction = transactions_repository
        .create_transaction(
            user.id,
            &payload.ticker,
            payload.quantity,
            payload.price.clone(),
            "sell",
        )
        .await?;

    // Update user balance (add the proceeds from sale)
    let sale_proceeds = &payload.price * payload.quantity;
    let new_balance = user.balance + sale_proceeds;
    users_repository
        .update_user_balance(user.id, new_balance)
        .await?;

    // Update holding quantity
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

#[derive(Debug, Deserialize, Validate)]
struct CreateBuyTransactionRequest {
    #[validate(length(min = 1, max = 10))]
    ticker: String,
    #[validate(range(min = 1, max = 10000))]
    quantity: i32,
    price: BigDecimal,
}

#[derive(Debug, Deserialize, Validate)]
struct CreateSellTransactionRequest {
    #[validate(length(min = 1, max = 10))]
    ticker: String,
    #[validate(range(min = 1, max = 10000))]
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
