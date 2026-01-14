# 🔐 Auth System Rust

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

> Complete, secure, modular authentication system **database-agnostic** built with Rust and Axum.

---

## Table of Contents

- [Overview](#-overview)
- [Features](#-features)
- [Architecture](#-architecture)
- [Quick Start](#-quick-start)
- [Database Configuration](#-database-configuration)
- [API Endpoints](#-api-endpoints)
- [Project Structure](#-project-structure)
- [Security](#-security)
- [How to Use in Other Projects](#-how-to-use-in-other-projects)
- [Examples](#-examples)
- [Testing](#-testing)
- [Contributing](#-contributing)
- [License](#-license)

---

## Overview

This is a **production-ready** authentication system that can be easily integrated into any Rust project. The key feature is **complete database independence**, allowing you to choose (or switch) databases without changing a single line of business logic code.

### Why use this system?

- ✅ **Database Agnostic** - Use PostgreSQL, MySQL, SQLite, MongoDB or even in-memory
- ✅ **Security First** - Argon2 for password hashing, JWT for tokens
- ✅ **Modular and Reusable** - Clone and use in any project
- ✅ **Type-Safe** - Leverage Rust's type safety
- ✅ **Async/Await** - Maximum performance with Tokio
- ✅ **Production Ready** - Robust error handling
- ✅ **Easy to Extend** - Add new databases by implementing a trait

---

## ✨ Features

### Authentication

- User registration with validation
- Login with username/password
- JWT tokens (JSON Web Tokens)
- Route protection via middleware
- Tokens with expiration (24 hours by default)

### Security

- Password hashing with **Argon2** (OWASP recommended)
- JWT signed with HMAC-SHA256
- Passwords never returned in responses
- Uniqueness validation (unique email and username)

### Database

- **In-Memory** - For development and testing
- **PostgreSQL** - Robust relational database
- **MySQL** - Compatible with MariaDB
- **SQLite** - Local database
- **MongoDB** - NoSQL document-based

### Architecture

- **Repository Pattern** - Complete decoupling
- **Trait-based** - Extensible and testable
- **Async/Await** - Performance with Tokio
- **Modular** - Use only what you need

---

## Arquitetura

```
┌─────────────────────────────────────────────────────┐
│             HTTP Layer (Axum Handlers)              │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────┐ │
│  │  Register   │  │    Login     │  │  Protected │ │
│  │   Handler   │  │   Handler    │  │   Routes   │ │
│  └─────────────┘  └──────────────┘  └────────────┘ │
└────────────────────────┬────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────┐
│           Business Logic Layer (Services)            │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────┐ │
│  │  JWT Service │  │Crypto Service│  │Auth Logic │ │
│  └──────────────┘  └──────────────┘  └───────────┘ │
└────────────────────────┬────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────┐
│        Repository Layer (UserRepository Trait)       │
│                   Trait-based Abstraction            │
└────────────────────────┬────────────────────────────┘
                         │
            ┌────────────┼────────────┐
            │            │            │
            ▼            ▼            ▼
     ┌──────────┐ ┌──────────┐ ┌──────────┐
     │PostgreSQL│ │  MySQL   │ │ MongoDB  │
     │   Impl   │ │   Impl   │ │   Impl   │
     └──────────┘ └──────────┘ └──────────┘
```

---

## Quick Start

### Prerequisites

- Rust 1.70 or higher
- (Optional) Database of your choice

### Installation

```bash
# Clone the repository
git clone https://github.com/seu-usuario/auth-system-rust.git
cd auth-system-rust

# Copy the example .env file
cp .env.example .env

# Edit .env and configure your JWT_SECRET
# You can generate one with: openssl rand -base64 32
nano .env
```

### Run with In-Memory (no database)

```bash
# Compile and run
cargo run

# The server will start at http://0.0.0.0:3000
```

### Test the Endpoints

```bash
# 1. Register a new user
curl -X POST http://localhost:3000/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john",
    "email": "john@email.com",
    "password": "Password123!"
  }'

# Response: {"token":"eyJ0eXAiOiJKV1QiLCJhbGc..."}

# 2. Login
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john",
    "password": "Password123!"
  }'

# 3. Access protected route (use the received token)
curl -X GET http://localhost:3000/private \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"

# Response: "Access granted for user: <user_id>"
```

---

## 💾 Database Configuration

### Option 1: In-Memory (Default)

**Ideal for:** Development, testing, prototypes

**Configuration:** None! It's ready to use.

**Warning:** Data is lost when the process ends.

```rust
// Already configured in main.rs
let user_repo = Arc::new(InMemoryUserRepository::new());
```

---

### Option 2: PostgreSQL

**Ideal for:** Production, robust applications

#### 1. Enable the feature in Cargo.toml

```toml
[features]
default = ["postgres"]
```

#### 2. Configure .env

```env
JWT_SECRET=your_secret_here
DATABASE_URL=postgresql://user:password@localhost/auth_db
```

#### 3. Create the database and table

```sql
CREATE DATABASE auth_db;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
```

#### 4. Uncomment the code in main.rs

See the commented section at the end of the `src/main.rs` file and uncomment the PostgreSQL block.

---

### Option 3: MySQL

**Ideal for:** Applications already using MySQL/MariaDB

#### 1. Enable the feature

```toml
[features]
default = ["mysql"]
```

#### 2. Configure .env

```env
DATABASE_URL=mysql://user:password@localhost/auth_db
```

#### 3. Create the table

```sql
CREATE TABLE users (
    id CHAR(36) PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);
```

#### 4. Uncomment the MySQL code in main.rs

---

### Option 4: SQLite

**Ideal for:** Desktop applications, small projects

#### 1. Enable the feature

```toml
[features]
default = ["sqlite"]
```

#### 2. Configure .env

```env
DATABASE_URL=sqlite://auth.db
```

#### 3. Create the table

```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    is_active INTEGER DEFAULT 1
);
```

#### 4. Uncomment the SQLite code in main.rs

---

### Option 5: MongoDB

**Ideal for:** NoSQL applications, unstructured data

#### 1. Enable the feature

```toml
[features]
default = ["mongodb"]
```

#### 2. Configure o .env

```env
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=auth_db
```

#### 3. No need to create a table!

MongoDB creates the collection automatically. Optionally, you can create indexes for better performance:

```bash
# Option A: Run setup script (creates indexes)
cargo run --example mongodb_setup --features mongodb

# Option B: MongoDB creates everything automatically on first use
# Simply run the application!
```

**Note:** MongoDB is schema-less (no fixed schema), so it doesn't need migrations like SQL databases.

#### 4. Uncomment the MongoDB code in main.rs

---

## 📡 API Endpoints

### POST /register

Register a new user.

**Request Body:**

```json
{
  "username": "john",
  "email": "john@email.com",
  "password": "Password123!"
}
```

**Response (201 Created):**

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

**Errors:**

- `409 Conflict` - User already exists
- `500 Internal Server Error` - Processing error

---

### POST /login

Authenticate an existing user.

**Request Body:**

```json
{
  "username": "john",
  "password": "Password123!"
}
```

**Response (200 OK):**

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

**Errors:**

- `401 Unauthorized` - Invalid credentials

---

### GET /private

Protected route (requires authentication).

**Headers:**

```
Authorization: Bearer <your_jwt_token>
```

**Response (200 OK):**

```
Access granted for user: <user_id>
```

**Errors:**

- `401 Unauthorized` - Invalid, expired or missing token

---

## 📂 Project Structure

```
auth-system/
├── Cargo.toml                # Dependencies and configurations
├── .env                      # Environment variables (do not commit!)
├── .env.example              # Configuration example
├── README.md                 # This documentation
│
├── src/
│   ├── lib.rs                # Main library (AppState)
│   ├── main.rs               # Entry point (HTTP server)
│   ├── errors.rs             # Custom error types
│   │
│   ├── auth/                 # Authentication module
│   │   ├── mod.rs
│   │   ├── crypto.rs         # Hash/verification of passwords (Argon2)
│   │   ├── jwt.rs            # JWT creation/validation
│   │   └── extractor.rs      # Authenticated user extractor (Axum)
│   │
│   ├── db/                   # Database layer
│   │   ├── mod.rs
│   │   ├── user_repository.rs         # Trait (interface)
│   │   ├── memory_connection.rs       # In-memory implementation
│   │   ├── postgres_connection.rs     # PostgreSQL implementation
│   │   ├── mysql_connection.rs        # MySQL implementation
│   │   ├── sqlite_connection.rs       # SQLite implementation
│   │   └── mongodb_connection.rs      # MongoDB implementation
│   │
│   ├── models/               # Data models
│   │   ├── mod.rs
│   │   ├── user.rs           # User, CreateUser
│   │   └── auth.rs           # LoginRequest, RegisterRequest, LoginResponse
│   │
│   └── handlers/             # HTTP Handlers
│       ├── mod.rs
│       └── auth_handler.rs   # register_handler, login_handler
│
└── migrations/               # SQL migrations (optional)
    └── 001_create_users.sql
```

---

## 🔒 Security

### Password Hashing

We use **Argon2**, winner of the Password Hashing Competition and recommended by OWASP:

- ✅ Resistant to brute force attacks
- ✅ Resistant to GPU/ASIC attacks
- ✅ Unique salt per password
- ✅ Secure settings by default

### JWT Tokens

- ✅ Signed with HMAC-SHA256
- ✅ Expires in 24 hours (configurable)
- ✅ Contains only the user ID (no sensitive data)
- ✅ Validated on each request

### Best Practices

1. **Never commit `.env`** - Add to `.gitignore`
2. **Use strong secrets** - Generate with `openssl rand -base64 32`
3. **HTTPS in production** - Use TLS/SSL
4. **Rate limiting** - Add brute force protection
5. **Input validation** - Always validate user data

---

## 🔄 How to Use in Other Projects

### Method 1: Clone and Customize

1. Clone this repository to your project
2. Choose the database (see configuration section)
3. Customize models and handlers as needed
4. Run and develop!

### Method 2: As Local Dependency

```toml
# Your project/Cargo.toml
[dependencies]
auth-system = { path = "../auth-system" }
```

```rust
// Your project/src/main.rs
use auth_system::{AppState, handlers::auth_handler};
use auth_system::db::postgres_connection::PostgresUserRepository;

#[tokio::main]
async fn main() {
    // Configure your database
    let user_repo = Arc::new(PostgresUserRepository::new(pool));

    let state = AppState {
        jwt_secret: "...".into(),
        user_repo,
    };

    // Use the ready-made handlers!
    let app = Router::new()
        .route("/register", post(auth_handler::register_handler))
        .route("/login", post(auth_handler::login_handler));
}
```

### Method 3: Create Custom Implementation

```rust
// Your project/src/db/custom_repository.rs
use async_trait::async_trait;
use auth_system::db::user_repository::UserRepository;

struct MyRepository {
    // Your implementation
}

#[async_trait]
impl UserRepository for MyRepository {
    // Implement the methods
    async fn create(...) -> Result<User, AuthError> {
        // Your logic
    }
    // ...
}
```

---

## 📝 Examples

### Example 1: Complete API with PostgreSQL

```rust
use auth_system::{AppState, handlers::auth_handler};
use auth_system::db::postgres_connection::PostgresUserRepository;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user_repo = Arc::new(PostgresUserRepository::new(db_pool));

    let state = AppState {
        jwt_secret: std::env::var("JWT_SECRET").unwrap(),
        user_repo,
    };

    let app = Router::new()
        .route("/register", post(auth_handler::register_handler))
        .route("/login", post(auth_handler::login_handler))
        .route("/profile", get(profile_handler))  // Custom handler
        .with_state(state);

    // ... server
}

// Custom handler that uses AuthUser
async fn profile_handler(user: AuthUser) -> Json<UserProfile> {
    // user.user_id contains the authenticated user's ID
    // Fetch additional data and return
    Json(UserProfile { /* ... */ })
}
```

### Example 2: Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use auth_system::db::memory_connection::InMemoryUserRepository;

    #[tokio::test]
    async fn test_register_success() {
        let user_repo = Arc::new(InMemoryUserRepository::new());
        let state = AppState {
            jwt_secret: "test_secret".into(),
            user_repo,
        };

        let request = RegisterRequest {
            username: "test".into(),
            email: "test@test.com".into(),
            password: "Password123!".into(),
        };

        let result = register_handler(State(state), Json(request)).await;
        assert!(result.is_ok());
    }
}
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture

# Test specific feature
cargo test --features postgres
```

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a branch for your feature (`git checkout -b feature/MyFeature`)
3. Commit your changes (`git commit -m 'Add MyFeature'`)
4. Push to the branch (`git push origin feature/MyFeature`)
5. Open a Pull Request

### Areas to Contribute

- [ ] Add more databases (Redis, DynamoDB, etc)
- [ ] Implement refresh tokens
- [ ] Add 2FA (Two-Factor Authentication)
- [ ] Rate limiting
- [ ] Email verification
- [ ] Password reset
- [ ] OAuth2 integration
- [ ] GraphQL support

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - SQL toolkit
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - JWT implementation
- [argon2](https://github.com/RustCrypto/password-hashes) - Password hashing

---

## 📞 Support

If you have problems or questions:

1. Check the [documentation](#-table-of-contents)
2. Search for [existing issues](https://github.com/seu-usuario/auth-system-rust/issues)
3. Open a [new issue](https://github.com/seu-usuario/auth-system-rust/issues/new)

---

## 🎓 Learn More

- [Rust Documentation](https://doc.rust-lang.org/)
- [Axum Documentation](https://docs.rs/axum/)
- [JWT.io](https://jwt.io/)
- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)

---

<div align="center">

**Made with ❤️ and Rust 🦀**

[⬆ Back to top](#-auth-system-rust)

</div>
