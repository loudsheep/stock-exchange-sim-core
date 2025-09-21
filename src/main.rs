use crate::{errors::not_found_handler, ws::handler::ws_handler};

pub use self::errors::{Error, Result};
use axum::{Extension, Router, routing::get};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{EnvFilter, fmt};

mod auth;
mod config;
mod errors;
mod grpc;
mod models;
mod repository;
mod routes;
mod services;
mod ws;

use config::Config;

/// Application state containing shared resources
///
/// This struct holds all shared application resources including
/// database connections, Redis pool, and configuration.
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL connection pool
    pub pg_pool: Arc<PgPool>,
    /// Redis connection pool for caching and session management
    pub redis_pool: Arc<bb8::Pool<bb8_redis::RedisConnectionManager>>,
    /// Application configuration
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::from_env()?;

    // Initialize tracing with proper level filtering
    fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!(
                "stock_exchange_sim_core={},tower_http=debug",
                config.log_level
            )
            .into()
        }))
        .init();

    tracing::info!("Starting Stock Exchange Simulator API");
    tracing::info!("Log level: {}", config.log_level);

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(config.max_db_connections)
        .connect(&config.database_url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create database pool: {}", e);
            e
        })?;

    tracing::info!("Database connected successfully");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to run migrations: {}", e);
            e
        })?;

    tracing::info!("Database migrations completed");

    // Create Redis pool
    let manager = bb8_redis::RedisConnectionManager::new(config.redis_url.clone())?;
    let redis_pool = bb8::Pool::builder().build(manager).await.map_err(|e| {
        tracing::error!("Failed to create Redis pool: {}", e);
        e
    })?;

    tracing::info!("Redis connected successfully");

    let state = AppState {
        pg_pool: Arc::new(pool),
        redis_pool: Arc::new(redis_pool),
        config: config.clone(),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, stock-sim!" }))
        .route("/health", get(health_check))
        .route("/ws", get(ws_handler))
        .merge(routes::routes())
        .layer(Extension(state))
        .fallback(not_found_handler)
        .into_make_service();

    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    tracing::info!("Server listening on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

/// Health check endpoint
///
/// Returns "OK" if the service is running properly.
/// This endpoint is useful for load balancers and monitoring systems.
async fn health_check() -> &'static str {
    "OK"
}
