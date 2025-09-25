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

[Simple random price generation](https://github.com/loudsheep/stock-exchange-sim-prices)

To connect your own gRPC server:

1. Implement the `PriceFeed` service interface
2. Configure the `GRPC_SERVER_URL` environment variable
3. Enable TLS if your server requires it: `GRPC_TLS_ENABLED=true`
4. Ensure your server streams price updates for ticker "ALL" to update all prices

## 📄 License

This project is licensed under the AGPL License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Database operations with [SQLx](https://github.com/launchbadge/sqlx)
- Authentication using [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- Password hashing with [Argon2](https://github.com/RustCrypto/password-hashes)
- Validation powered by [validator](https://github.com/Keats/validator)

---

**Note**: This is the core trading service. For real-time price feeds, connect it to your gRPC price feed server.