use axum::Router;

mod auth;
mod balance;
mod transactions;

pub fn routes() -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/balance", balance::routes())
        .nest("/transactions", transactions::routes())
}
