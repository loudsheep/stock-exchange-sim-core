use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    dotenvy::dotenv().ok();

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

    let app = Router::new().route("/", get(|| async { "Hello, stock-sim!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
