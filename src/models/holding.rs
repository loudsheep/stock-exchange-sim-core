use bigdecimal::BigDecimal;

#[derive(sqlx::FromRow, Debug)]
pub struct Holding {
    pub id: i32,
    pub user_id: i32,
    pub ticker: String,
    pub quantity: i32,
    pub average_price: BigDecimal,
}