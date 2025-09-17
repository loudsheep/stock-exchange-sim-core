use bigdecimal::BigDecimal;
use sqlx::PgPool;

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

    pub async fn get_transactions_by_user(&self, user_id: i32) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT id, user_id, ticker, quantity, price, transaction_type
            FROM transactions
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(transactions)
    }

    pub async fn get_transaction_by_id(&self, transaction_id: i32) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            SELECT id, user_id, ticker, quantity, price, transaction_type
            FROM transactions
            WHERE id = $1
            "#,
            transaction_id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(transaction)
    }
}
