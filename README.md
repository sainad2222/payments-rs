# Payments Backend Service

A RESTful API backend service for managing transactions and user accounts in a simplified payment system.

## Features

- User registration and authentication with JWT
- Account management
- Transaction processing (deposits, withdrawals, transfers)
- Balance tracking
- Rate limiting
- Comprehensive error handling

## Tech Stack

- Rust programming language
- Axum web framework
- PostgreSQL database via Deadpool connection pool
- JWT-based authentication
- Argon2 password hashing
- Docker and Docker Compose for containerization

## Prerequisites

- Rust (1.70.0+)
- Docker and Docker Compose
- PostgreSQL client (optional, for direct database access)

## Getting Started

### Running with Docker Compose

The easiest way to run the application is using Docker Compose:

```bash
# Clone the repository
git clone <repository-url>
cd payments-backend

# Start the application and database
docker-compose up -d

# The API will be available at http://localhost:3002
```

### Manual Setup

To run the application without Docker:

1. Install Rust and Cargo (https://rustup.rs/)
2. Set up a PostgreSQL database
3. Create a `.env` file based on `.env.example`
4. Run the application:

```bash
cargo run
```

## Environment Variables

Configure the application using these environment variables:

- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT token generation
- `JWT_EXPIRATION`: Token expiration time in seconds (default: 86400)
- `PORT`: HTTP server port (default: 3002)
- `RUST_LOG`: Logging level (default: debug)

## API Documentation

### Authentication

#### Register a new user

```
POST /api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "username": "username",
  "password": "password123",
  "full_name": "John Doe"
}
```

#### Login

```
POST /api/auth/login
Content-Type: application/json

{
  "username_or_email": "username",
  "password": "password123"
}
```

Response includes JWT token to be used in subsequent requests:

```json
{
  "token": "your.jwt.token",
  "user": {
    "id": "user-uuid",
    "email": "user@example.com",
    "username": "username",
    "full_name": "John Doe",
    "created_at": "2023-01-01T00:00:00Z"
  }
}
```

### User Management

#### Get user profile

```
GET /api/users/me
Authorization: Bearer <your-jwt-token>
```

#### Update user profile

```
PUT /api/users/me
Authorization: Bearer <your-jwt-token>
Content-Type: application/json

{
  "email": "newemail@example.com",
  "username": "newusername",
  "full_name": "New Name"
}
```

### Account Management

#### Create an account

```
POST /api/accounts
Authorization: Bearer <your-jwt-token>
Content-Type: application/json

{
  "currency": "USD"
}
```

#### List user accounts

```
GET /api/accounts
Authorization: Bearer <your-jwt-token>
```

#### Get account details

```
GET /api/accounts/{account_id}
Authorization: Bearer <your-jwt-token>
```

### Transaction Management

#### Create transaction

```
POST /api/transactions
Authorization: Bearer <your-jwt-token>
Content-Type: application/json

{
  "source_account_id": "source-account-uuid",  // Optional for deposits
  "destination_account_id": "dest-account-uuid",  // Optional for withdrawals
  "amount": "100.00",
  "currency": "USD",
  "transaction_type": "transfer",  // "deposit", "withdrawal", or "transfer"
  "description": "Payment for services"
}
```

#### List transactions

```
GET /api/transactions?page=1&page_size=10
Authorization: Bearer <your-jwt-token>
```

#### Get transaction details

```
GET /api/transactions/{transaction_id}
Authorization: Bearer <your-jwt-token>
```

## Development

### Running Tests

```bash
cargo test
```

### Database Migrations

Database migrations are automatically applied when using Docker Compose. The SQL files are located in the `migrations` directory.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
