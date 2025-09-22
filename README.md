# Stock Exchange Simulator Core

A robust, high-performance REST API for simulating stock trading operations, built with Rust, Axum, PostgreSQL, and Redis. This core service provides authentication, balance management, trading operations, and real-time price updates via WebSocket connections.

## 🚀 Features

### Core Trading Features
- 💰 **Balance Management** - Secure deposit and withdrawal operations with precise decimal handling
- 📈 **Stock Trading** - Buy and sell operations with real-time price validation
- 📊 **Portfolio Management** - Track holdings with automatic average price calculations
- 📋 **Transaction History** - Complete audit trail of all trading activities

### Real-time Features
- 🔄 **WebSocket Support** - Real-time price updates for subscribed tickers
- 📡 **gRPC Integration** - Connects to external price feed service for live market data
- ⚡ **Redis Caching** - High-performance price caching and session management

### Security & Reliability
- 🔐 **JWT Authentication** - Secure token-based authentication with configurable expiration
- 🛡️ **Password Security** - Argon2 password hashing with salt
- ✅ **Input Validation** - Comprehensive request validation and sanitization
- 🚫 **SQL Injection Protection** - Compile-time verified queries with SQLx
- 📝 **Audit Logging** - Security event logging for monitoring
- 🔒 **Error Handling** - Sanitized error responses preventing information disclosure

### Architecture & Performance
- 🏗️ **Clean Architecture** - Modular design with clear separation of concerns
- 🔗 **Connection Pooling** - Efficient database and Redis connection management
- 📏 **Request Limits** - Configurable request size limits for DDoS protection
- 🎯 **Type Safety** - Rust's strong type system prevents runtime errors

## 🏗️ Architecture

The application follows a clean architecture pattern with clear separation of concerns:

```
src/
├── auth/              # Authentication & authorization
│   ├── jwt.rs         # JWT token handling
│   ├── password.rs    # Password hashing/verification
│   └── mod.rs
├── config/            # Configuration management
│   └── config.rs      # Environment-based configuration
├── errors/            # Centralized error handling
│   └── errors.rs      # Error types and HTTP responses
├── grpc/              # gRPC client integration
│   └── mod.rs         # Price feed service client
├── models/            # Database models
├── repository/        # Data access layer
│   ├── user_repository.rs
│   ├── transaction_repository.rs
│   ├── holdings_repository.rs
│   └── mod.rs
├── routes/            # HTTP route handlers
│   ├── auth.rs        # Authentication endpoints
│   ├── balance.rs     # Balance management
│   ├── transactions.rs # Trading operations
│   ├── holdings.rs    # Portfolio management
│   └── mod.rs
├── services/          # Business logic services
├── ws/                # WebSocket handlers
│   └── handler.rs     # Real-time price updates
└── main.rs           # Application entry point
```

## 🔧 API Endpoints

### Authentication
- `POST /auth/register` - Register a new user account
  ```json
  {
    "email": "user@example.com",
    "password": "secure_password"
  }
  ```
- `POST /auth/login` - Authenticate and receive JWT token
  ```json
  {
    "email": "user@example.com", 
    "password": "secure_password"
  }
  ```
- `POST /auth/logout` - Logout user (token invalidation)

### Balance Management
- `GET /balance/` - Get current account balance
- `POST /balance/deposit` - Deposit funds
  ```json
  {
    "amount": 1000.50
  }
  ```
- `POST /balance/withdraw` - Withdraw funds
  ```json
  {
    "amount": 500.25
  }
  ```

### Trading Operations
- `GET /transactions/` - Get transaction history
- `POST /transactions/buy` - Execute buy order
  ```json
  {
    "ticker": "AAPL",
    "quantity": 10
  }
  ```
- `POST /transactions/sell` - Execute sell order
  ```json
  {
    "ticker": "AAPL", 
    "quantity": 5
  }
  ```

### Portfolio Management
- `GET /holdings/` - Get current stock holdings

### Real-time Data
- `GET /ws` - WebSocket endpoint for real-time price updates
  - Send: `subscribe:AAPL` to receive price updates
  - Receive: `update:AAPL:150.25` format

### System Health
- `GET /health` - Health check endpoint
- `GET /` - Service status

## 🔐 Security Features

### Authentication & Authorization
- **JWT Tokens**: Secure, stateless authentication with configurable expiration
- **Password Security**: Argon2 hashing with cryptographically secure salt generation
- **Token Validation**: Comprehensive JWT validation with proper error handling
- **Session Management**: Secure session handling via JWT tokens

### Input Security
- **Request Validation**: All inputs validated using the `validator` crate
- **Size Limits**: Configurable maximum request sizes prevent DoS attacks
- **Input Sanitization**: Ticker symbols and user inputs properly sanitized
- **SQL Injection Protection**: Compile-time SQL verification with SQLx
- **Type Safety**: Rust's type system prevents many common vulnerabilities

### Error Handling & Information Security
- **Information Disclosure Prevention**: Generic error messages for external users
- **Comprehensive Logging**: Detailed error logging for debugging without exposure
- **Structured Responses**: Consistent error response format with timestamps
- **Audit Trail**: Security events logged for monitoring and compliance

### Infrastructure Security
- **Database Security**: Parameterized queries and connection pooling
- **Redis Security**: Secure connection handling and data validation
- **gRPC Security**: Optional TLS support for external service connections
- **Security Headers**: Middleware available for CSRF, XSS, and clickjacking protection

### Security Middleware (Available for Production)
```rust
// Add to main.rs for production deployments:
.layer(axum::middleware::from_fn(security::security_headers))
.layer(axum::middleware::from_fn(security::request_timeout))
```

Security headers include:
- `X-Frame-Options: DENY` (Clickjacking protection)
- `X-Content-Type-Options: nosniff` (MIME sniffing protection)
- `X-XSS-Protection: 1; mode=block` (XSS protection)
- `Strict-Transport-Security` (HTTPS enforcement)
- `Content-Security-Policy` (Content injection protection)
- `Referrer-Policy` (Information disclosure prevention)

## 🛠️ Setup & Installation

### Prerequisites

- **Rust** 1.70+ with 2024 edition support
- **PostgreSQL** 13+ for persistent data storage
- **Redis** 6+ for caching and session management
- **Protocol Buffers** compiler for gRPC support

### Installation Steps

1. **Clone the repository**
   ```bash
   git clone https://github.com/loudsheep/stock-exchange-sim-core.git
   cd stock-exchange-sim-core
   ```

2. **Install system dependencies**
   ```bash
   # Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install protobuf-compiler postgresql redis-server
   
   # macOS
   brew install protobuf postgresql redis
   ```

3. **Setup environment configuration**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration values
   ```

4. **Install Rust dependencies**
   ```bash
   cargo build
   ```

5. **Database setup**
   ```bash
   # Create database
   createdb stock_exchange_sim
   
   # Run migrations
   sqlx database create
   sqlx migrate run
   ```

6. **Start supporting services**
   ```bash
   # Start Redis
   redis-server
   
   # Start PostgreSQL (if not running as service)
   pg_ctl start
   ```

7. **Run the application**
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:3000`

## ⚙️ Configuration

### Environment Variables

#### Required Configuration
```bash
# Database connection
DATABASE_URL=postgresql://user:password@localhost:5432/stock_exchange_sim

# Redis connection  
REDIS_URL=redis://localhost:6379

# gRPC price feed service
GRPC_SERVER_URL=http://localhost:50051

# JWT secret (minimum 32 characters)
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production-must-be-at-least-32-chars
```

#### Optional Configuration
```bash
# Server settings
SERVER_HOST=127.0.0.1          # Default: 127.0.0.1
SERVER_PORT=3000               # Default: 3000
MAX_REQUEST_SIZE=1048576       # Default: 1MB

# Database settings
MAX_DB_CONNECTIONS=5           # Default: 5

# Security settings  
JWT_EXPIRATION_HOURS=24        # Default: 24 hours
GRPC_TLS_ENABLED=false         # Default: false

# Logging
LOG_LEVEL=info                 # Default: info
RUST_LOG=stock_exchange_sim_core=info,tower_http=debug
```

### Database Configuration

The application uses PostgreSQL with the following schema:

- **users**: User accounts with encrypted passwords and balances
- **transactions**: Complete trading history with audit trail
- **holdings**: Current user positions with average cost basis

### Redis Configuration

Redis is used for:
- Real-time price data caching
- Session management 
- WebSocket connection state
- Rate limiting data (future enhancement)

## 🔌 gRPC Price Feed Integration

This core service integrates with an external gRPC server for real-time price data.

### Expected gRPC Service Interface

```proto
syntax = "proto3";
package pricefeed;

service PriceFeed {
  rpc GetPrice(PriceRequest) returns (PriceResponse);
  rpc StreamPrices(PriceRequest) returns (stream PriceResponse);
}

message PriceRequest {
  string ticker = 1;
}

message PriceResponse {
  string ticker = 1;
  double price = 2;
  int64 timestamp = 3;
}
```

### Connecting Your gRPC Server

**[PLACEHOLDER: Link to your gRPC price feed server repository will go here]**

To connect your own gRPC server:

1. Implement the `PriceFeed` service interface
2. Configure the `GRPC_SERVER_URL` environment variable
3. Enable TLS if your server requires it: `GRPC_TLS_ENABLED=true`
4. Ensure your server streams price updates for ticker "ALL" to update all prices

## 🧪 Development

### Development Workflow

1. **Code Quality**
   ```bash
   # Format code
   cargo fmt
   
   # Run linter
   cargo clippy --all-targets --all-features
   
   # Type checking
   cargo check
   ```

2. **Testing**
   ```bash
   # Run tests
   cargo test
   
   # Run tests with output
   cargo test -- --nocapture
   ```

3. **Database Development**
   ```bash
   # Create new migration
   sqlx migrate add create_new_table
   
   # Run migrations
   sqlx migrate run
   
   # Revert last migration
   sqlx migrate revert
   ```

### Development Tools

- **SQLx CLI**: Database migration management
- **Cargo Watch**: Automatic rebuilds during development
- **Rust Analyzer**: IDE support for enhanced development experience

### Adding New Features

1. Define models in `src/models/`
2. Create repository methods in `src/repository/`
3. Implement business logic in `src/services/`
4. Add route handlers in `src/routes/`
5. Update tests and documentation

## 🚀 Production Deployment

### Security Checklist

- [ ] Generate strong JWT secret (32+ characters)
- [ ] Enable TLS for gRPC connections in production
- [ ] Configure proper database credentials
- [ ] Set up monitoring and logging
- [ ] Enable security middleware in main.rs
- [ ] Configure rate limiting (future enhancement)
- [ ] Review and harden network security
- [ ] Set up backup strategies for database
- [ ] Configure HTTPS with proper TLS certificates
- [ ] Set up database connection encryption
- [ ] Implement log rotation and monitoring
- [ ] Configure environment-specific secrets management

### Security Best Practices

#### Environment Security
```bash
# Use strong, unique secrets
JWT_SECRET=$(openssl rand -base64 32)

# Enable TLS for production
GRPC_TLS_ENABLED=true

# Limit request sizes appropriately
MAX_REQUEST_SIZE=1048576  # 1MB default

# Set reasonable JWT expiration
JWT_EXPIRATION_HOURS=24
```

#### Application Security
```rust
// Enable security middleware in production
let app = Router::new()
    .merge(routes::routes())
    .layer(axum::middleware::from_fn(security::security_headers))
    .layer(axum::middleware::from_fn(security::request_timeout))
    .layer(Extension(state));
```

#### Database Security
- Use connection pooling with appropriate limits
- Enable SSL/TLS for database connections
- Implement database connection timeouts
- Use read-only replicas for read operations
- Regular security updates for PostgreSQL

#### Redis Security  
- Enable authentication on Redis instance
- Use TLS for Redis connections in production
- Configure appropriate timeout values
- Monitor Redis memory usage and performance

### Performance Optimization

- [ ] Configure appropriate connection pool sizes
- [ ] Set up Redis clustering for high availability
- [ ] Configure database read replicas if needed
- [ ] Monitor and optimize query performance
- [ ] Set up proper caching strategies

### Monitoring & Observability

- Monitor JWT token validation failures
- Track database connection pool usage
- Monitor gRPC service availability
- Log authentication and authorization events
- Track trading volume and error rates

## 📋 Data Models

### User Model
```rust
struct User {
    id: i32,
    email: String,        // Unique email address
    password: String,     // Argon2 hashed password
    balance: BigDecimal,  // Account balance with precision
    created_at: DateTime<Utc>,
}
```

### Transaction Model
```rust
struct Transaction {
    id: i32,
    user_id: i32,                    // Foreign key to users
    ticker: String,                  // Stock symbol
    quantity: i32,                   // Number of shares
    price: BigDecimal,               // Price per share
    transaction_type: String,        // "buy" or "sell"
    created_at: DateTime<Utc>,
}
```

### Holding Model
```rust
struct Holding {
    id: i32,
    user_id: i32,                    // Foreign key to users
    ticker: String,                  // Stock symbol
    quantity: i32,                   // Current shares held
    average_price: BigDecimal,       // Average cost basis
    updated_at: DateTime<Utc>,
}
```

## 🤝 Contributing

### Code Standards

- Follow Rust standard formatting (`cargo fmt`)
- Pass all clippy lints (`cargo clippy`)
- Include comprehensive tests for new features
- Update documentation for API changes
- Follow security best practices

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Implement your changes with tests
4. Ensure all checks pass
5. Update documentation as needed
6. Submit a pull request with clear description

### Issue Reporting

When reporting issues, please include:
- Rust version and target platform
- Complete error messages and stack traces
- Steps to reproduce the issue
- Expected vs actual behavior

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Database operations with [SQLx](https://github.com/launchbadge/sqlx)
- Authentication using [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- Password hashing with [Argon2](https://github.com/RustCrypto/password-hashes)
- Validation powered by [validator](https://github.com/Keats/validator)

---

**Note**: This is the core trading service. For real-time price feeds, connect it to your gRPC price feed server. **[Link to gRPC server repository will be added here]**