use std::sync::Arc;
use auth_system::{auth::extractor::AuthUser, db::memory_connection::InMemoryUserRepository};
use auth_system::handlers::auth_handler;
use auth_system::AppState;
use tokio::net::TcpListener;
use axum::{Router, routing::{get, post}};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let jwt_secret =  std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");
    let user_repo = Arc::new(InMemoryUserRepository::new());
    
    let state = AppState { 
        jwt_secret, 
        user_repo,
    };

    let app = Router::new()
        .route("/register", post(auth_handler::register_handler))
        .route("/login", post(auth_handler::login_handler))
        .route("/private", get(protect_handler))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");


    println!("Auth System running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.expect("Failed to start server");
}


async fn protect_handler(user: AuthUser) -> String {
    format!("Access granted for user: {}", user.user_id)
}



// ==================================================================================
// 🔧 EXAMPLES OF CONFIGURATION FOR OTHER DATABASES
// ==================================================================================
// Uncomment and configure the database you want to use.
// Remember to:
// 1. Enable the corresponding feature in Cargo.toml
// 2. Add the necessary dependencies
// 3. Configure the environment variables in .env
// ==================================================================================

// ┌─────────────────────────────────────────────────────────────────────────────┐
// │ 🐘 POSTGRESQL                                                               │
// └─────────────────────────────────────────────────────────────────────────────┘
// 
// 1. Adicione ao Cargo.toml:
//    [features]
//    postgres = ["sqlx"]
//    
//    [dependencies.sqlx]
//    version = "0.8"
//    features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"]
//    optional = true
//
// 2. Configure in .env:
//    DATABASE_URL=postgresql://user:password@localhost/auth_db
//
// 3. Uncomment the code below:

/*
use auth_system::db::postgres_connection::PostgresUserRepository;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Create PostgreSQL connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");
    
    let user_repo = Arc::new(PostgresUserRepository::new(db_pool));
    
    let state = AppState { jwt_secret, user_repo };
    
    // ... rest of code is the same
}
*/

// ┌─────────────────────────────────────────────────────────────────────────────┐
// │ 🐬 MYSQL                                                                    │
// └─────────────────────────────────────────────────────────────────────────────┘
//
// 1. Adicione ao Cargo.toml:
//    [features]
//    mysql = ["sqlx"]
//    
//    [dependencies.sqlx]
//    version = "0.8"
//    features = ["runtime-tokio-rustls", "mysql", "uuid", "chrono"]
//    optional = true
//
// 2. Configure in .env:
//    DATABASE_URL=mysql://user:password@localhost/auth_db
//
// 3. Uncomment the code below:

/*
use auth_system::db::mysql_connection::MySQLUserRepository;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Create MySQL connection pool
    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to MySQL");
    
    let user_repo = Arc::new(MySQLUserRepository::new(db_pool));
    
    let state = AppState { jwt_secret, user_repo };
    
    // ... rest of code is the same
}
*/

// ┌─────────────────────────────────────────────────────────────────────────────┐
// │ 📦 SQLITE                                                                   │
// └─────────────────────────────────────────────────────────────────────────────┘
//
// 1. Adicione ao Cargo.toml:
//    [features]
//    sqlite = ["sqlx"]
//    
//    [dependencies.sqlx]
//    version = "0.8"
//    features = ["runtime-tokio-rustls", "sqlite", "uuid", "chrono"]
//    optional = true
//
// 2. Configure no .env:
//    DATABASE_URL=sqlite://auth.db
//
// 3. Descomente o código abaixo:

/*
use auth_system::db::sqlite_connection::SQLiteUserRepository;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Create SQLite connection pool
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to SQLite");
    
    let user_repo = Arc::new(SQLiteUserRepository::new(db_pool));
    
    let state = AppState { jwt_secret, user_repo };
    
    // ... rest of code is the same
}
*/

// ┌─────────────────────────────────────────────────────────────────────────────┐
// │ 🍃 MONGODB                                                                  │
// └─────────────────────────────────────────────────────────────────────────────┘
//
// 1. Adicione ao Cargo.toml:
//    [features]
//    mongodb = ["dep:mongodb"]
//    
//    [dependencies]
//    mongodb = { version = "3.1", optional = true }
//    bson = { version = "2.13", features = ["chrono-0_4", "uuid-1"], optional = true }
//
// 2. Configure no .env:
//    MONGODB_URI=mongodb://localhost:27017
//    MONGODB_DATABASE=auth_db
//
// 3. Descomente o código abaixo:

/*
use auth_system::db::mongodb_connection::MongoDBUserRepository;
use mongodb::Client;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let mongodb_database = std::env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE must be set");
    
    // Conecta ao MongoDB
    let client = Client::with_uri_str(&mongodb_uri)
        .await
        .expect("Failed to connect to MongoDB");
    
    let user_repo = Arc::new(MongoDBUserRepository::new(client, &mongodb_database));
    
    let state = AppState { jwt_secret, user_repo };
    
    // ... rest of code is the same
}
*/

