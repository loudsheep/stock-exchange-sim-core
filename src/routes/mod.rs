use axum::Router;

use crate::AppState;
mod auth;

pub fn routes(state: AppState) -> Router {
    Router::new().nest("/auth", auth::routes(state))
}
