// Export database modules

/// Main trait that defines repository operations
pub mod user_repository;

/// In-memory implementation (for development and testing)
pub mod memory_connection;

/// PostgreSQL implementation (optional - feature "postgres")
#[cfg(feature = "postgres")]
pub mod postgres_connection;

/// MySQL implementation (optional - feature "mysql")
#[cfg(feature = "mysql")]
pub mod mysql_connection;

/// SQLite implementation (optional - feature "sqlite")
#[cfg(feature = "sqlite")]
pub mod sqlite_connection;

/// MongoDB implementation (optional - feature "mongodb")
#[cfg(feature = "mongodb")]
pub mod mongodb_connection;