use sqlx::types::BigDecimal;

use crate::{Error, Result, models::user::User};

pub struct UserRepository<'a> {
    pool: &'a sqlx::PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, email: &str, password: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password, balance)
            VALUES ($1, $2, 1000)
            RETURNING id, email, password, balance
            "#,
            email,
            password
        )
        .fetch_one(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, balance
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, balance
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(user)
    }

    pub async fn update_user_balance(&self, user_id: i32, new_balance: BigDecimal) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET balance = $1
            WHERE id = $2
            "#,
            new_balance,
            user_id
        )
        .execute(self.pool)
        .await
        .map_err(Error::Database)?;

        Ok(())
    }
}
