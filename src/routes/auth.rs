use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new().route("/login", post(login))
}

async fn login() -> &'static str {
    "Login endpoint"
}