use crate::errors::not_found_handler;

pub use self::errors::{Error, Result};
use axum::{Extension, Router, routing::get};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber;

mod auth;
mod errors;
mod models;
mod repository;
mod routes;
mod services;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
    pub redis_pool: Arc<bb8::Pool<bb8_redis::RedisConnectionManager>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations.");

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = bb8_redis::RedisConnectionManager::new(redis_url)?;
    let redis_pool = bb8::Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create Redis pool.");

    let state = AppState {
        pool: Arc::new(pool),
        redis_pool: Arc::new(redis_pool),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, stock-sim!" }))
        .merge(routes::routes())
        .layer(Extension(state))
        .fallback(not_found_handler)
        .into_make_service();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
