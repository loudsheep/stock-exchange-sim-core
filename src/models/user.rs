use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub balance: BigDecimal,
}