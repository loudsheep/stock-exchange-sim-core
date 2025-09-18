# Stock Exchange Simulator API

A robust REST API for simulating stock trading operations built with Rust, Axum, and PostgreSQL.

## Features

- 🔐 **JWT-based Authentication** - Secure user authentication with proper password hashing
- 💰 **Balance Management** - Deposit and withdraw funds with validation
- 📈 **Stock Trading** - Buy and sell stocks with real-time balance updates
- 📊 **Portfolio Management** - Track holdings with average price calculations
- 🛡️ **Input Validation** - Comprehensive validation for all API inputs
- 🏗️ **Industry Standards** - Follows Rust best practices and security standards

## API Endpoints

### Authentication
- `POST /auth/register` - Register a new user
- `POST /auth/login` - Authenticate user and get JWT token
- `POST /auth/logout` - Logout user (invalidate token)

### Balance Management
- `GET /balance/` - Get current user balance
- `POST /balance/deposit` - Deposit funds to account
- `POST /balance/withdraw` - Withdraw funds from account

### Trading
- `GET /transactions/` - Get user's transaction history
- `POST /transactions/buy` - Execute buy order
- `POST /transactions/sell` - Execute sell order

### Portfolio
- `GET /holdings/` - Get user's current holdings

### System
- `GET /health` - Health check endpoint

## Environment Variables

Create a `.env` file based on `.env.example`:

```bash
# Database
DATABASE_URL=postgresql://username:password@localhost/stock_exchange_sim

# Redis (for caching/sessions)
REDIS_URL=redis://localhost:6379

# Security
JWT_SECRET=your-super-secret-jwt-key-here

# Server Configuration (optional)
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
MAX_DB_CONNECTIONS=5
LOG_LEVEL=info
```

## Development

### Prerequisites

- Rust 1.70+
- PostgreSQL 13+
- Redis 6+

### Setup

1. Clone the repository
2. Install dependencies:
   ```bash
   cargo build
   ```

3. Set up the database:
   ```bash
   # Create database and run migrations
   sqlx database create
   sqlx migrate run
   ```

4. Start the server:
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:3000`

### Development Tools

- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy`
- **Type Checking**: `cargo check`

## Security Features

- ✅ **Password Hashing** - Argon2 password hashing
- ✅ **Input Validation** - Comprehensive request validation
- ✅ **JWT Authentication** - Secure token-based auth
- ✅ **Error Handling** - No information leakage in errors
- ✅ **SQL Injection Protection** - Parameterized queries with SQLx
- ✅ **Rate Limiting Ready** - Structure supports rate limiting implementation

## Architecture

The application follows a clean architecture pattern:

```
src/
├── auth/           # Authentication & authorization
├── config/         # Configuration management
├── errors/         # Error handling & types
├── models/         # Database models
├── repository/     # Data access layer
├── routes/         # HTTP route handlers
└── services/       # Business logic services
```

## Data Models

### User
- `id` - Unique user identifier
- `email` - User email (unique)
- `password` - Hashed password
- `balance` - Account balance (BigDecimal for precision)

### Transaction
- `id` - Transaction identifier
- `user_id` - Reference to user
- `ticker` - Stock symbol
- `quantity` - Number of shares
- `price` - Price per share
- `transaction_type` - "buy" or "sell"

### Holding
- `id` - Holding identifier
- `user_id` - Reference to user
- `ticker` - Stock symbol
- `quantity` - Number of shares held
- `average_price` - Average purchase price

## License

This project is licensed under the MIT License.