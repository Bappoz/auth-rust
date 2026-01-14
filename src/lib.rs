pub mod auth;
pub mod handlers;
pub mod models;
pub mod errors;
pub mod db;


use std::sync::Arc;
use crate::db::user_repository::UserRepository;

#[derive(Clone)]
pub struct AppState {
    /// Segredo usado para assinar e verificar JWT tokens
    pub jwt_secret: String,
    
    /// Repositório de usuários (trait object)
    /// Permite usar qualquer implementação de UserRepository
    /// (PostgreSQL, MongoDB, In-Memory, etc)
    pub user_repo: Arc<dyn UserRepository>,
}