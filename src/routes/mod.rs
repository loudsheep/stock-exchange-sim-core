use axum::Router;

mod auth;

pub fn routes() -> Router {
    Router::new().nest("/auth", auth::routes())
}
