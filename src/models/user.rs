use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub balance: f64,
}