//! # Configuration Management
//!
//! This module handles application configuration loading from environment variables
//! with proper validation and default values.

use serde::Deserialize;
use std::env;

/// Application configuration structure
///
/// Contains all configurable parameters for the application.
/// All values are loaded from environment variables with sensible defaults.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// PostgreSQL database connection URL
    pub database_url: String,
    /// Redis connection URL for caching
    pub redis_url: String,
    /// gRPC server URL for price feed
    pub grpc_server_url: String,
    /// JWT signing secret key
    pub jwt_secret: String,
    /// Server host address
    pub server_host: String,
    /// Server port number
    pub server_port: u16,
    /// Maximum number of database connections in the pool
    pub max_db_connections: u32,
    /// Application log level (trace, debug, info, warn, error)
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Returns
    ///
    /// Returns a `Config` instance with values loaded from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing or invalid.
    ///
    /// # Required Environment Variables
    ///
    /// - `DATABASE_URL`: PostgreSQL connection string
    /// - `REDIS_URL`: Redis connection string  
    /// - `JWT_SECRET`: Secret key for JWT signing
    ///
    /// # Optional Environment Variables
    ///
    /// - `SERVER_HOST`: Server host (default: "127.0.0.1")
    /// - `SERVER_PORT`: Server port (default: 3000)
    /// - `MAX_DB_CONNECTIONS`: Max DB connections (default: 5)
    /// - `LOG_LEVEL`: Log level (default: "info")
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable is required"))?,
            redis_url: env::var("REDIS_URL")
                .map_err(|_| anyhow::anyhow!("REDIS_URL environment variable is required"))?,
            grpc_server_url: env::var("GRPC_SERVER_URL")
                .map_err(|_| anyhow::anyhow!("GRPC_SERVER_URL environment variable is required"))?,
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| anyhow::anyhow!("JWT_SECRET environment variable is required"))?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid SERVER_PORT"))?,
            max_db_connections: env::var("MAX_DB_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid MAX_DB_CONNECTIONS"))?,
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        })
    }
}
