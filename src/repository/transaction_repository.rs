use sqlx::{PgPool, types::BigDecimal};

use crate::{Error, Result, models::transaction::Transaction};

pub struct TransactionRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TransactionRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        TransactionRepository { pool }
    }

    pub async fn create_transaction(
        &self,
        user_id: i32,
        ticker: &str,
        quantity: i32,
        price: BigDecimal,
        transaction_type: &str,
    ) -> Result<Transaction> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            INSERT INTO transactions (user_id, ticker, quantity, price, transaction_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, ticker, quantity, price, transaction_type
            "#,
            user_id,
            ticker,
            quantity,
            price,
            transaction_type
        )
        .fetch_one(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(transaction)
    }
}
