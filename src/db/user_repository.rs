use async_trait::async_trait;
use crate::models::user::{User, CreateUser};
use crate::errors::AuthError;
use uuid::Uuid;

/// Trait that defines user repository operations
/// 
/// This trait allows you to decouple the business logic from the database.
/// You can implement this trait for any database:
/// - PostgreSQL
/// - MySQL
/// - MongoDB
/// - SQLite
/// - In-Memory (for testing)
/// - etc.
/// 
/// Handlers only use this trait, without knowing which bank is being used.
/// 
#[async_trait]
pub trait UserRepository: Send + Sync {
    //Create a new user
    async fn create(&self, user: CreateUser, password_hash: String) -> Result<User, AuthError>;

    // Search user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;

    // Search user by username
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AuthError>;

    // Search user by Id
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError>;
}