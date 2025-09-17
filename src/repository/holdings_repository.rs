use bigdecimal::BigDecimal;
use sqlx::PgPool;

use crate::{Error, Result, models::holding::Holding};

pub struct HoldingsRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> HoldingsRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        HoldingsRepository { pool }
    }

    pub async fn get_holdings_by_user(&self, user_id: i32) -> Result<Vec<Holding>> {
        let holdings = sqlx::query_as!(
            Holding,
            r#"
            SELECT id, user_id, ticker, quantity, average_price
            FROM holdings
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(holdings)
    }

    pub async fn get_holding_by_user_and_ticker(
        &self,
        user_id: i32,
        ticker: &str,
    ) -> Result<Option<Holding>> {
        let holding = sqlx::query_as!(
            Holding,
            r#"
            SELECT id, user_id, ticker, quantity, average_price
            FROM holdings
            WHERE user_id = $1 AND ticker = $2
            "#,
            user_id,
            ticker
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(holding)
    }

    pub async fn create_holding(
        &self,
        user_id: i32,
        ticker: &str,
        quantity: i32,
        average_price: BigDecimal,
    ) -> Result<Holding> {
        let holding = sqlx::query_as!(
            Holding,
            r#"
            INSERT INTO holdings (user_id, ticker, quantity, average_price)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, ticker, quantity, average_price
            "#,
            user_id,
            ticker,
            quantity,
            average_price
        )
        .fetch_one(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(holding)
    }

    pub async fn update_holding(
        &self,
        holding_id: i32,
        quantity: i32,
        average_price: BigDecimal,
    ) -> Result<Holding> {
        let holding = sqlx::query_as!(
            Holding,
            r#"
            UPDATE holdings
            SET quantity = $1, average_price = $2
            WHERE id = $3
            RETURNING id, user_id, ticker, quantity, average_price
            "#,
            quantity,
            average_price,
            holding_id
        )
        .fetch_one(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(holding)
    }
}
