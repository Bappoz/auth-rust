// Exporta os módulos de banco de dados

/// Trait principal que define as operações de repositório
pub mod user_repository;

/// Implementação in-memory (para desenvolvimento e testes)
pub mod memory_connection;

/// Implementação PostgreSQL (opcional - feature "postgres")
#[cfg(feature = "postgres")]
pub mod postgres_connection;

/// Implementação MySQL (opcional - feature "mysql")
#[cfg(feature = "mysql")]
pub mod mysql_connection;

/// Implementação SQLite (opcional - feature "sqlite")
#[cfg(feature = "sqlite")]
pub mod sqlite_connection;

/// Implementação MongoDB (opcional - feature "mongodb")
#[cfg(feature = "mongodb")]
pub mod mongodb_connection;