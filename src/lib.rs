pub mod auth;
pub mod handlers;
pub mod models;
pub mod errors;
pub mod db;


use std::sync::Arc;
use crate::db::user_repository::UserRepository;

#[derive(Clone)]
pub struct AppState {
    /// Secret used to sign and verify JWT tokens
    pub jwt_secret: String,
    
    /// User repository (trait object)
    /// Allows using any UserRepository implementation
    /// (PostgreSQL, MongoDB, In-Memory, etc)
    pub user_repo: Arc<dyn UserRepository>,
}