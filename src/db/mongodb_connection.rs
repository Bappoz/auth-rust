/// Implementação MongoDB do UserRepository
/// 
/// Este arquivo só é compilado se a feature "mongodb" estiver habilitada.
/// 
/// Para usar:
/// 1. Adicione ao Cargo.toml:
///    mongodb = "3.1"
///    bson = { version = "2.13", features = ["chrono-0_4", "uuid-1"] }
/// 
/// 2. Configure o MONGODB_URI no .env:
///    MONGODB_URI=mongodb://localhost:27017
///    MONGODB_DATABASE=auth_db
/// 
/// 3. O MongoDB criará a collection automaticamente

#[cfg(feature = "mongodb")]
use async_trait::async_trait;
#[cfg(feature = "mongodb")]
use mongodb::{Client, Collection};
#[cfg(feature = "mongodb")]
use mongodb::bson::doc;
#[cfg(feature = "mongodb")]
use uuid::Uuid;
#[cfg(feature = "mongodb")]
use chrono::Utc;
#[cfg(feature = "mongodb")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "mongodb")]
use crate::{
    db::user_repository::UserRepository,
    models::user::{User, CreateUser},
    errors::AuthError,
};

#[cfg(feature = "mongodb")]
#[derive(Debug, Serialize, Deserialize)]
struct UserDocument {
    #[serde(rename = "_id")]
    id: String,
    username: String,
    email: String,
    password_hash: String,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
    is_active: bool,
}

#[cfg(feature = "mongodb")]
pub struct MongoDBUserRepository {
    collection: Collection<UserDocument>,
}

#[cfg(feature = "mongodb")]
impl MongoDBUserRepository {
    pub fn new(client: Client, database_name: &str) -> Self {
        let collection = client.database(database_name).collection("users");
        Self { collection }
    }
}

#[cfg(feature = "mongodb")]
#[async_trait]
impl UserRepository for MongoDBUserRepository {
    async fn create(&self, user: CreateUser, password_hash: String) -> Result<User, AuthError> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let doc = UserDocument {
            id: id.to_string(),
            username: user.username.clone(),
            email: user.email.clone(),
            password_hash: password_hash.clone(),
            created_at: now,
            updated_at: now,
            is_active: true,
        };

        self.collection
            .insert_one(doc)
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
        let doc = self.collection
            .find_one(doc! { "email": email })
            .await
            .map_err(|_| AuthError::DatabaseError)?;

        Ok(doc.map(|d| User {
            id: Uuid::parse_str(&d.id).unwrap(),
            username: d.username,
            email: d.email,
            password_hash: d.password_hash,
            created_at: d.created_at,
            updated_at: d.updated_at,
            is_active: d.is_active,
        }))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AuthError> {
        let doc = self.collection
            .find_one(doc! { "username": username })
            .await
            .map_err(|_| AuthError::DatabaseError)?;

        Ok(doc.map(|d| User {
            id: Uuid::parse_str(&d.id).unwrap(),
            username: d.username,
            email: d.email,
            password_hash: d.password_hash,
            created_at: d.created_at,
            updated_at: d.updated_at,
            is_active: d.is_active,
        }))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        let doc = self.collection
            .find_one(doc! { "_id": id.to_string() })
            .await
            .map_err(|_| AuthError::DatabaseError)?;

        Ok(doc.map(|d| User {
            id: Uuid::parse_str(&d.id).unwrap(),
            username: d.username,
            email: d.email,
            password_hash: d.password_hash,
            created_at: d.created_at,
            updated_at: d.updated_at,
            is_active: d.is_active,
        }))
    }
}
