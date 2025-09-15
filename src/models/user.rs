use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub balance: BigDecimal,
}