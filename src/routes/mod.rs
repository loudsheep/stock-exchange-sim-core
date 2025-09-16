use axum::Router;

use crate::AppState;
mod auth;

pub fn routes() -> Router {
    Router::new().nest("/auth", auth::routes())
}
