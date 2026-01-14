use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use crate::{
    db::user_repository::UserRepository,
    models::user::{User, CreateUser},
    errors::AuthError,
};

/// Implementação in-memory do UserRepository
/// 
/// Armazena os usuários em um HashMap na memória.
/// Útil para:
/// - Desenvolvimento local
/// - Testes unitários
/// - Protótipos rápidos
/// 
/// AVISO: Os dados são perdidos quando o processo termina!
#[derive(Clone)]
pub struct InMemoryUserRepository {
    /// HashMap theread-safe that stores users (Key, Value)
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    // Create a new instance for repository in-memory
    pub fn new() -> Self {
        Self{
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}


impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}


#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create(&self, user: CreateUser, password_hash: String) -> Result<User, AuthError> {
        let mut users = self.users.lock().unwrap();

        // Generates a new UUID
        let id: Uuid = Uuid::new_v4();

        // Create a user
        let new_user = User {   
            id, 
            username: user.username,
            email: user.email,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_active: true,
        };

        // Insert HashMap
        users.insert(id.to_string(), new_user.clone());

        Ok(new_user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError> {
        let users = self.users.lock().unwrap();
        
        // Linear search for email(Not eficient, but ok for testing)
        Ok(users.values().find(|u| u.email == email).cloned())
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AuthError> {
        let users = self.users.lock().unwrap();
        
        // Linear search for username
        Ok(users.values().find(|u| u.username == username).cloned())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        let users = self.users.lock().unwrap();
        
        // Direct search for ID (O(1))
        Ok(users.get(&id.to_string()).cloned())
    }
}