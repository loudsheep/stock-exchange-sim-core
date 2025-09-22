//! Security-focused tests for the stock exchange API

#[cfg(test)]
mod security_tests {
    use super::*;
    
    #[test]
    fn test_jwt_secret_validation() {
        // Test that JWT secret validation works correctly
        std::env::set_var("DATABASE_URL", "postgresql://test");
        std::env::set_var("REDIS_URL", "redis://test");
        std::env::set_var("GRPC_SERVER_URL", "http://test");
        
        // Test with short JWT secret (should fail)
        std::env::set_var("JWT_SECRET", "short");
        let result = crate::config::Config::from_env();
        assert!(result.is_err());
        
        // Test with long enough JWT secret (should pass)
        std::env::set_var("JWT_SECRET", "this-is-a-very-long-secure-jwt-secret-key-for-testing");
        let result = crate::config::Config::from_env();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_password_hashing_security() {
        use crate::auth::password::{hash_password, verify_password};
        
        let password = "test_password_123";
        
        // Test that hashing produces different results each time (salt randomization)
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        assert_ne!(hash1, hash2);
        
        // Test that verification works correctly
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
        
        // Test that wrong password fails
        assert!(!verify_password("wrong_password", &hash1).unwrap());
    }
    
    #[test]
    fn test_ticker_validation() {
        // Test valid tickers
        assert!(is_valid_ticker_format("AAPL"));
        assert!(is_valid_ticker_format("MSFT"));
        assert!(is_valid_ticker_format("A"));
        assert!(is_valid_ticker_format("ABCDEFGHIJ")); // 10 chars max
        
        // Test invalid tickers
        assert!(!is_valid_ticker_format("")); // empty
        assert!(!is_valid_ticker_format("ABCDEFGHIJK")); // too long
        assert!(!is_valid_ticker_format("AAP-L")); // special characters
        assert!(!is_valid_ticker_format("AAP L")); // spaces
        assert!(!is_valid_ticker_format("aap")); // this depends on implementation
    }
    
    // Helper function to test ticker validation logic
    fn is_valid_ticker_format(ticker: &str) -> bool {
        if ticker.is_empty() || ticker.len() > 10 {
            return false;
        }
        ticker.chars().all(|c| c.is_ascii_alphanumeric())
    }
    
    #[test]
    fn test_request_validation_limits() {
        // Test that validation limits are reasonable
        use validator::Validate;
        use crate::routes::transactions::{CreateBuyTransactionRequest, CreateSellTransactionRequest};
        use serde_json::json;
        
        // Valid request
        let valid_buy = CreateBuyTransactionRequest {
            ticker: "AAPL".to_string(),
            quantity: 10,
        };
        assert!(valid_buy.validate().is_ok());
        
        // Invalid requests
        let invalid_ticker = CreateBuyTransactionRequest {
            ticker: "".to_string(), // empty ticker
            quantity: 10,
        };
        assert!(invalid_ticker.validate().is_err());
        
        let invalid_quantity = CreateBuyTransactionRequest {
            ticker: "AAPL".to_string(),
            quantity: 0, // zero quantity
        };
        assert!(invalid_quantity.validate().is_err());
        
        let too_many_shares = CreateBuyTransactionRequest {
            ticker: "AAPL".to_string(),
            quantity: 20000, // exceeds limit
        };
        assert!(too_many_shares.validate().is_err());
    }
}

// Note: These tests need to be added to the actual test modules in each file
// This is a demonstration of security-focused testing patterns