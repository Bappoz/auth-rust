/// SQLite implementation of UserRepository
/// 
/// This file is only compiled if the "sqlite" feature is enabled.
/// 
/// To use:
/// 1. Add to Cargo.toml:
///    sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "uuid", "chrono"] }
/// 
/// 2. Configure DATABASE_URL in .env:
///    DATABASE_URL=sqlite://auth.db
/// 
/// 3. Create the table:
///    CREATE TABLE users (
///        id TEXT PRIMARY KEY,
///        username TEXT UNIQUE NOT NULL,
///        email TEXT UNIQUE NOT NULL,
///        password_hash TEXT NOT NULL,
///        created_at TEXT NOT NULL,
///        updated_at TEXT NOT NULL,
///        is_active INTEGER DEFAULT 1
///    );

#[cfg(feature = "sqlite")]
use async_trait::async_trait;
#[cfg(feature = "sqlite")]
use sqlx::SqlitePool;
#[cfg(feature = "sqlite")]
use uuid::Uuid;
#[cfg(feature = "sqlite")]
use chrono::Utc;
#[cfg(feature = "sqlite")]
use crate::{
    db::user_repository::UserRepository,
    models::user::{User, CreateUser},
    errors::AuthError,
};

#[cfg(feature = "sqlite")]
pub struct SQLiteUserRepository {
    pool: SqlitePool,
}

#[cfg(feature = "sqlite")]
impl SQLiteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[cfg(feature = "sqlite")]
#[async_trait]
impl UserRepository for SQLiteUserRepository {
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
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(1)
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
        let result = sqlx::query_as::<_, (String, String, String, String, String, String, i32)>(
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
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at).unwrap().with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at).unwrap().with_timezone(&Utc),
            is_active: is_active != 0,
        }))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AuthError> {
        let result = sqlx::query_as::<_, (String, String, String, String, String, String, i32)>(
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
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at).unwrap().with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at).unwrap().with_timezone(&Utc),
            is_active: is_active != 0,
        }))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        let result = sqlx::query_as::<_, (String, String, String, String, String, String, i32)>(
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
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at).unwrap().with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at).unwrap().with_timezone(&Utc),
            is_active: is_active != 0,
        }))
    }
}
