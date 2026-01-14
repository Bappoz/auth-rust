/// MySQL implementation of UserRepository
/// 
/// This file is only compiled if the "mysql" feature is enabled.
/// 
/// To use:
/// 1. Add to Cargo.toml:
///    sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "mysql", "uuid", "chrono"] }
/// 
/// 2. Configure DATABASE_URL in .env:
///    DATABASE_URL=mysql://user:password@localhost/auth_db
/// 
/// 3. Create the table:
///    CREATE TABLE users (
///        id CHAR(36) PRIMARY KEY,
///        username VARCHAR(50) UNIQUE NOT NULL,
///        email VARCHAR(255) UNIQUE NOT NULL,
///        password_hash TEXT NOT NULL,
///        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
///        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
///        is_active BOOLEAN DEFAULT TRUE
///    );

#[cfg(feature = "mysql")]
use async_trait::async_trait;
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "mysql")]
use uuid::Uuid;
#[cfg(feature = "mysql")]
use chrono::Utc;
#[cfg(feature = "mysql")]
use crate::{
    db::user_repository::UserRepository,
    models::user::{User, CreateUser},
    errors::AuthError,
};

#[cfg(feature = "mysql")]
pub struct MySQLUserRepository {
    pool: MySqlPool,
}

#[cfg(feature = "mysql")]
impl MySQLUserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[cfg(feature = "mysql")]
#[async_trait]
impl UserRepository for MySQLUserRepository {
    async fn create(&self, user: CreateUser, password_hash: String) -> Result<User, AuthError> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, created_at, updated_at, is_active)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(id.to_string())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&password_hash)
        .bind(now)
        .bind(now)
        .bind(true)
        .execute(&self.pool)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(User {
            id,
            username: user.username,
            email: user.email,
            password_hash,
            created_at: now,
            updated_at: now,
            is_active: true,
        })
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError> {
        let result = sqlx::query_as::<_, (String, String, String, String, chrono::DateTime<Utc>, chrono::DateTime<Utc>, bool)>(
            "SELECT id, username, email, password_hash, created_at, updated_at, is_active FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(result.map(|(id, username, email, password_hash, created_at, updated_at, is_active)| User {
            id: Uuid::parse_str(&id).unwrap(),
            username,
            email,
            password_hash,
            created_at,
            updated_at,
            is_active,
        }))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AuthError> {
        let result = sqlx::query_as::<_, (String, String, String, String, chrono::DateTime<Utc>, chrono::DateTime<Utc>, bool)>(
            "SELECT id, username, email, password_hash, created_at, updated_at, is_active FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(result.map(|(id, username, email, password_hash, created_at, updated_at, is_active)| User {
            id: Uuid::parse_str(&id).unwrap(),
            username,
            email,
            password_hash,
            created_at,
            updated_at,
            is_active,
        }))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        let result = sqlx::query_as::<_, (String, String, String, String, chrono::DateTime<Utc>, chrono::DateTime<Utc>, bool)>(
            "SELECT id, username, email, password_hash, created_at, updated_at, is_active FROM users WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(result.map(|(id, username, email, password_hash, created_at, updated_at, is_active)| User {
            id: Uuid::parse_str(&id).unwrap(),
            username,
            email,
            password_hash,
            created_at,
            updated_at,
            is_active,
        }))
    }
}
