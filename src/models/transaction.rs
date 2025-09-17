use bigdecimal::BigDecimal;

#[derive(sqlx::FromRow, Debug)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub ticker: String,
    pub quantity: i32,
    pub price: BigDecimal,
    pub transaction_type: String,
}
