use axum::{Extension, Json, Router, routing::get};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::{
    AppState, Result,
    auth::jwt::Claims,
    repository::{holdings_repository::HoldingsRepository, user_repository::UserRepository},
};

pub fn routes() -> Router {
    Router::new().route("/", get(get_holdings))
}

async fn get_holdings(
    claims: Claims,
    db: Extension<AppState>,
) -> Result<Json<Vec<HoldingResponse>>> {
    let users_repository = UserRepository::new(&db.pg_pool);
    let holdings_repository = HoldingsRepository::new(&db.pg_pool);

    let user = users_repository.get_user_by_id(claims.user_id).await?;
    let user = user.ok_or(crate::Error::Unauthorized)?;

    let holdings = holdings_repository.get_holdings_by_user(user.id).await?;

    let response: Vec<HoldingResponse> = holdings
        .into_iter()
        .map(|h| HoldingResponse {
            id: h.id,
            ticker: h.ticker,
            quantity: h.quantity,
            average_price: h.average_price,
        })
        .collect();

    Ok(Json(response))
}

#[derive(Serialize, Deserialize)]
struct HoldingResponse {
    id: i32,
    ticker: String,
    quantity: i32,
    average_price: BigDecimal,
}
