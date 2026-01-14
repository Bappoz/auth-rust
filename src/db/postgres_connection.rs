// Este arquivo só é compilado se a feature "postgres" estiver habilitada
#[cfg(feature = "postgres")]
use async_trait::async_trait;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
#[cfg(feature = "postgres")]
use uuid::Uuid;
#[cfg(feature = "postgres")]
use crate::{
    db::user_repository::UserRepository,
    models::user::{User, CreateUser},
    errors::AuthError,
};

/// Implementação PostgreSQL do UserRepository
/// 
/// Usa SQLx para acessar o banco de dados PostgreSQL.
/// Requer:
/// - PostgreSQL instalado e rodando
/// - Tabela 'users' criada (veja migrations)
/// - Feature "postgres" habilitada no Cargo.toml
#[cfg(feature = "postgres")]
pub struct PostgresUserRepository {
    pool: PgPool,
}

#[cfg(feature = "postgres")]
impl PostgresUserRepository {
    /// Cria uma nova instância do repositório PostgreSQL
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(feature = "postgres")]
#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: CreateUser, password_hash: String) -> Result<User, AuthError> {
        let id = Uuid::new_v4();
        
        // Query SQL para inserir o usuário
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash, created_at, updated_at, is_active)
            VALUES ($1, $2, $3, $4, NOW(), NOW(), true)
            RETURNING id, username, email, password_hash, created_at, updated_at, is_active
            "#,
            id,
            user.username,
            user.email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, password_hash, created_at, updated_at, is_active 
               FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AuthError> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, password_hash, created_at, updated_at, is_active 
               FROM users WHERE username = $1"#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, password_hash, created_at, updated_at, is_active 
               FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(user)
    }
}