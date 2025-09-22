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
    /// Maximum request body size in bytes (default: 1MB)
    pub max_request_size: usize,
    /// Enable TLS for gRPC connections
    pub grpc_tls_enabled: bool,
    /// JWT token expiration time in hours
    pub jwt_expiration_hours: i64,
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
    /// - `GRPC_SERVER_URL`: gRPC server URL for price feed
    /// - `JWT_SECRET`: Secret key for JWT signing (minimum 32 characters)
    ///
    /// # Optional Environment Variables
    ///
    /// - `SERVER_HOST`: Server host (default: "127.0.0.1")
    /// - `SERVER_PORT`: Server port (default: 3000)
    /// - `MAX_DB_CONNECTIONS`: Max DB connections (default: 5)
    /// - `LOG_LEVEL`: Log level (default: "info")
    /// - `MAX_REQUEST_SIZE`: Max request body size in bytes (default: 1048576)
    /// - `GRPC_TLS_ENABLED`: Enable TLS for gRPC (default: false)
    /// - `JWT_EXPIRATION_HOURS`: JWT token expiration in hours (default: 24)
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| anyhow::anyhow!("JWT_SECRET environment variable is required"))?;
        
        // Validate JWT secret strength (minimum 32 characters for security)
        if jwt_secret.len() < 32 {
            return Err(anyhow::anyhow!(
                "JWT_SECRET must be at least 32 characters long for security"
            ));
        }

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable is required"))?,
            redis_url: env::var("REDIS_URL")
                .map_err(|_| anyhow::anyhow!("REDIS_URL environment variable is required"))?,
            grpc_server_url: env::var("GRPC_SERVER_URL")
                .map_err(|_| anyhow::anyhow!("GRPC_SERVER_URL environment variable is required"))?,
            jwt_secret,
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
            max_request_size: env::var("MAX_REQUEST_SIZE")
                .unwrap_or_else(|_| "1048576".to_string()) // 1MB default
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid MAX_REQUEST_SIZE"))?,
            grpc_tls_enabled: env::var("GRPC_TLS_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid GRPC_TLS_ENABLED"))?,
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid JWT_EXPIRATION_HOURS"))?,
        })
    }
}
