#[derive(sqlx::FromRow, Debug)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub ticker: String,
    pub quantity: i32,
    pub price: f64,
    pub transaction_type: String,
    pub created_at: String,
    pub updated_at: String,
}
