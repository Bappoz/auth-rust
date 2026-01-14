-- Migration to create the users table in PostgreSQL
-- Execute with: psql -U user -d auth_db -f migrations/001_create_users_postgres.sql

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);

-- Indexes to improve search performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);

-- Comments
COMMENT ON TABLE users IS 'Authentication system users table';
COMMENT ON COLUMN users.id IS 'Unique user ID (UUID)';
COMMENT ON COLUMN users.username IS 'Username (unique)';
COMMENT ON COLUMN users.email IS 'User email (unique)';
COMMENT ON COLUMN users.password_hash IS 'Password hash (Argon2)';
