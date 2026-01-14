# 🔐 Auth System Rust

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

> Sistema de autenticação completo, seguro, modular e **agnóstico ao banco de dados** construído com Rust e Axum.

---

## Índice

- [Visão Geral](#-visão-geral)
- [Características](#-características)
- [Arquitetura](#-arquitetura)
- [Quick Start](#-quick-start)
- [Configuração de Banco de Dados](#-configuração-de-banco-de-dados)
- [Endpoints da API](#-endpoints-da-api)
- [Estrutura do Projeto](#-estrutura-do-projeto)
- [Segurança](#-segurança)
- [Como Usar em Outros Projetos](#-como-usar-em-outros-projetos)
- [Exemplos](#-exemplos)
- [Testes](#-testes)
- [Contribuindo](#-contribuindo)
- [Licença](#-licença)

---

## Visão Geral

Este é um sistema de autenticação **production-ready** que pode ser facilmente integrado em qualquer projeto Rust. O diferencial é a **total independência de banco de dados**, permitindo que você escolha (ou troque) o banco sem alterar nenhuma linha de código da lógica de negócio.

### Por que usar este sistema?

- ✅ **Agnóstico ao Banco de Dados** - Use PostgreSQL, MySQL, SQLite, MongoDB ou até in-memory
- ✅ **Segurança em Primeiro Lugar** - Argon2 para hash de senhas, JWT para tokens
- ✅ **Modular e Reutilizável** - Clone e use em qualquer projeto
- ✅ **Type-Safe** - Aproveite a segurança de tipos do Rust
- ✅ **Async/Await** - Performance máxima com Tokio
- ✅ **Pronto para Produção** - Tratamento robusto de erros
- ✅ **Fácil de Estender** - Adicione novos bancos implementando uma trait

---

## ✨ Características

### Autenticação

- Registro de usuários com validação
- Login com username/password
- JWT tokens (JSON Web Tokens)
- Proteção de rotas via middleware
- Tokens com expiração (24 horas por padrão)

### Segurança

- Hash de senhas com **Argon2** (recomendado pela OWASP)
- JWT assinado com HMAC-SHA256
- Senhas nunca retornadas nas respostas
- Validação de duplicidade (email e username únicos)

### Banco de Dados

- **In-Memory** - Para desenvolvimento e testes
- **PostgreSQL** - Banco relacional robusto
- **MySQL** - Compatível com MariaDB
- **SQLite** - Banco de dados local
- **MongoDB** - NoSQL document-based

### Arquitetura

- **Repository Pattern** - Desacoplamento total
- **Trait-based** - Extensível e testável
- **Async/Await** - Performance com Tokio
- **Modular** - Use apenas o que precisa

---

## Arquitetura

```
┌─────────────────────────────────────────────────────┐
│             HTTP Layer (Axum Handlers)              │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────┐ │
│  │  Register   │  │    Login     │  │  Protected │ │
│  │   Handler   │  │   Handler    │  │   Routes   │ │
│  └─────────────┘  └──────────────┘  └────────────┘ │
└────────────────────────┬────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────┐
│           Business Logic Layer (Services)            │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────┐ │
│  │  JWT Service │  │Crypto Service│  │Auth Logic │ │
│  └──────────────┘  └──────────────┘  └───────────┘ │
└────────────────────────┬────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────┐
│        Repository Layer (UserRepository Trait)       │
│                   Trait-based Abstraction            │
└────────────────────────┬────────────────────────────┘
                         │
            ┌────────────┼────────────┐
            │            │            │
            ▼            ▼            ▼
     ┌──────────┐ ┌──────────┐ ┌──────────┐
     │PostgreSQL│ │  MySQL   │ │ MongoDB  │
     │   Impl   │ │   Impl   │ │   Impl   │
     └──────────┘ └──────────┘ └──────────┘
```

---

## Quick Start

### Pré-requisitos

- Rust 1.70 ou superior
- (Opcional) Banco de dados de sua escolha

### Instalação

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/auth-system-rust.git
cd auth-system-rust

# Copie o arquivo de exemplo .env
cp .env.example .env

# Edite o .env e configure seu JWT_SECRET
# Você pode gerar um com: openssl rand -base64 32
nano .env
```

### Executar com In-Memory (sem banco)

```bash
# Compile e execute
cargo run

# O servidor iniciará em http://0.0.0.0:3000
```

### Testar os Endpoints

```bash
# 1. Registrar um novo usuário
curl -X POST http://localhost:3000/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "joao",
    "email": "joao@email.com",
    "password": "senha123"
  }'

# Resposta: {"token":"eyJ0eXAiOiJKV1QiLCJhbGc..."}

# 2. Login
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "joao",
    "password": "senha123"
  }'

# 3. Acessar rota protegida (use o token recebido)
curl -X GET http://localhost:3000/private \
  -H "Authorization: Bearer SEU_TOKEN_AQUI"

# Resposta: "Acesso concedido para o usuário: <user_id>"
```

---

## 💾 Configuração de Banco de Dados

### Opção 1: In-Memory (Padrão)

**Ideal para:** Desenvolvimento, testes, protótipos

**Configuração:** Nenhuma! Já está pronto para usar.

**Aviso:** Os dados são perdidos quando o processo termina.

```rust
// Já configurado na main.rs
let user_repo = Arc::new(InMemoryUserRepository::new());
```

---

### Opção 2: PostgreSQL

**Ideal para:** Produção, aplicações robustas

#### 1. Habilite a feature no Cargo.toml

```toml
[features]
default = ["postgres"]
```

#### 2. Configure o .env

```env
JWT_SECRET=seu_segredo_aqui
DATABASE_URL=postgresql://usuario:senha@localhost/auth_db
```

#### 3. Crie o banco e a tabela

```sql
CREATE DATABASE auth_db;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
```

#### 4. Descomente o código na main.rs

Veja a seção comentada no final do arquivo `src/main.rs` e descomente o bloco PostgreSQL.

---

### Opção 3: MySQL

**Ideal para:** Aplicações que já usam MySQL/MariaDB

#### 1. Habilite a feature

```toml
[features]
default = ["mysql"]
```

#### 2. Configure o .env

```env
DATABASE_URL=mysql://usuario:senha@localhost/auth_db
```

#### 3. Crie a tabela

```sql
CREATE TABLE users (
    id CHAR(36) PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);
```

#### 4. Descomente o código MySQL na main.rs

---

### Opção 4: SQLite

**Ideal para:** Aplicações desktop, projetos pequenos

#### 1. Habilite a feature

```toml
[features]
default = ["sqlite"]
```

#### 2. Configure o .env

```env
DATABASE_URL=sqlite://auth.db
```

#### 3. Crie a tabela

```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    is_active INTEGER DEFAULT 1
);
```

#### 4. Descomente o código SQLite na main.rs

---

### Opção 5: MongoDB

**Ideal para:** Aplicações NoSQL, dados não estruturados

#### 1. Habilite a feature

```toml
[features]
default = ["mongodb"]
```

#### 2. Configure o .env

```env
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=auth_db
```

#### 3. Não precisa criar tabela!

O MongoDB cria a collection automaticamente. Opcionalmente, você pode criar índices para melhor performance:

```bash
# Opção A: Executar script de setup (cria índices)
cargo run --example mongodb_setup --features mongodb

# Opção B: MongoDB cria tudo automaticamente no primeiro uso
# Simplesmente execute a aplicação!
```

**Nota:** MongoDB é schema-less (sem schema fixo), por isso não precisa de migrations como SQL databases.

#### 4. Descomente o código MongoDB na main.rs

---

## 📡 Endpoints da API

### POST /register

Registra um novo usuário.

**Request Body:**

```json
{
  "username": "joao",
  "email": "joao@email.com",
  "password": "senha123"
}
```

**Response (201 Created):**

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

**Errors:**

- `409 Conflict` - Usuário já existe
- `500 Internal Server Error` - Erro ao processar

---

### POST /login

Autentica um usuário existente.

**Request Body:**

```json
{
  "username": "joao",
  "password": "senha123"
}
```

**Response (200 OK):**

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

**Errors:**

- `401 Unauthorized` - Credenciais inválidas

---

### GET /private

Rota protegida (requer autenticação).

**Headers:**

```
Authorization: Bearer <seu_token_jwt>
```

**Response (200 OK):**

```
Acesso concedido para o usuário: <user_id>
```

**Errors:**

- `401 Unauthorized` - Token inválido, expirado ou ausente

---

## 📂 Estrutura do Projeto

```
auth-system/
├── Cargo.toml                # Dependências e configurações
├── .env                      # Variáveis de ambiente (não commitar!)
├── .env.example              # Exemplo de configuração
├── README.md                 # Esta documentação
│
├── src/
│   ├── lib.rs                # Biblioteca principal (AppState)
│   ├── main.rs               # Entry point (servidor HTTP)
│   ├── errors.rs             # Tipos de erro customizados
│   │
│   ├── auth/                 # Módulo de autenticação
│   │   ├── mod.rs
│   │   ├── crypto.rs         # Hash/verificação de senhas (Argon2)
│   │   ├── jwt.rs            # Criação/validação de JWT
│   │   └── extractor.rs      # Extrator de usuário autenticado (Axum)
│   │
│   ├── db/                   # Camada de banco de dados
│   │   ├── mod.rs
│   │   ├── user_repository.rs         # Trait (interface)
│   │   ├── memory_connection.rs       # Implementação in-memory
│   │   ├── postgres_connection.rs     # Implementação PostgreSQL
│   │   ├── mysql_connection.rs        # Implementação MySQL
│   │   ├── sqlite_connection.rs       # Implementação SQLite
│   │   └── mongodb_connection.rs      # Implementação MongoDB
│   │
│   ├── models/               # Modelos de dados
│   │   ├── mod.rs
│   │   ├── user.rs           # User, CreateUser
│   │   └── auth.rs           # LoginRequest, RegisterRequest, LoginResponse
│   │
│   └── handlers/             # HTTP Handlers
│       ├── mod.rs
│       └── auth_handler.rs   # register_handler, login_handler
│
└── migrations/               # SQL migrations (opcional)
    └── 001_create_users.sql
```

---

## 🔒 Segurança

### Hash de Senhas

Utilizamos **Argon2**, vencedor do Password Hashing Competition e recomendado pela OWASP:

- ✅ Resistente a ataques de força bruta
- ✅ Resistente a ataques de GPU/ASIC
- ✅ Salt único por senha
- ✅ Configurações seguras por padrão

### JWT Tokens

- ✅ Assinado com HMAC-SHA256
- ✅ Expira em 24 horas (configurável)
- ✅ Contém apenas o ID do usuário (sem dados sensíveis)
- ✅ Validado em cada requisição

### Boas Práticas

1. **Nunca commite o `.env`** - Adicione ao `.gitignore`
2. **Use secrets fortes** - Gere com `openssl rand -base64 32`
3. **HTTPS em produção** - Use TLS/SSL
4. **Rate limiting** - Adicione proteção contra brute force
5. **Validação de entrada** - Sempre valide dados do usuário

---

## 🔄 Como Usar em Outros Projetos

### Método 1: Clonar e Customizar

1. Clone este repositório para seu projeto
2. Escolha o banco de dados (veja seção de configuração)
3. Customize os modelos e handlers conforme necessário
4. Execute e desenvolva!

### Método 2: Como Dependência Local

```toml
# Seu projeto/Cargo.toml
[dependencies]
auth-system = { path = "../auth-system" }
```

```rust
// Seu projeto/src/main.rs
use auth_system::{AppState, handlers::auth_handler};
use auth_system::db::postgres_connection::PostgresUserRepository;

#[tokio::main]
async fn main() {
    // Configure seu banco
    let user_repo = Arc::new(PostgresUserRepository::new(pool));

    let state = AppState {
        jwt_secret: "...".into(),
        user_repo,
    };

    // Use os handlers prontos!
    let app = Router::new()
        .route("/register", post(auth_handler::register_handler))
        .route("/login", post(auth_handler::login_handler));
}
```

### Método 3: Criar Implementação Customizada

```rust
// Seu projeto/src/db/custom_repository.rs
use async_trait::async_trait;
use auth_system::db::user_repository::UserRepository;

struct MeuRepository {
    // Sua implementação
}

#[async_trait]
impl UserRepository for MeuRepository {
    // Implemente os métodos
    async fn create(...) -> Result<User, AuthError> {
        // Sua lógica
    }
    // ...
}
```

---

## 📝 Exemplos

### Exemplo 1: API Completa com PostgreSQL

```rust
use auth_system::{AppState, handlers::auth_handler};
use auth_system::db::postgres_connection::PostgresUserRepository;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user_repo = Arc::new(PostgresUserRepository::new(db_pool));

    let state = AppState {
        jwt_secret: std::env::var("JWT_SECRET").unwrap(),
        user_repo,
    };

    let app = Router::new()
        .route("/register", post(auth_handler::register_handler))
        .route("/login", post(auth_handler::login_handler))
        .route("/profile", get(profile_handler))  // Custom handler
        .with_state(state);

    // ... servidor
}

// Handler customizado que usa AuthUser
async fn profile_handler(user: AuthUser) -> Json<UserProfile> {
    // user.user_id contém o ID do usuário autenticado
    // Busque dados adicionais e retorne
    Json(UserProfile { /* ... */ })
}
```

### Exemplo 2: Testes Unitários

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use auth_system::db::memory_connection::InMemoryUserRepository;

    #[tokio::test]
    async fn test_register_success() {
        let user_repo = Arc::new(InMemoryUserRepository::new());
        let state = AppState {
            jwt_secret: "test_secret".into(),
            user_repo,
        };

        let request = RegisterRequest {
            username: "test".into(),
            email: "test@test.com".into(),
            password: "senha123".into(),
        };

        let result = register_handler(State(state), Json(request)).await;
        assert!(result.is_ok());
    }
}
```

---

## 🧪 Testes

```bash
# Rodar todos os testes
cargo test

# Rodar com output detalhado
cargo test -- --nocapture

# Testar feature específica
cargo test --features postgres
```

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/MinhaFeature`)
3. Commit suas mudanças (`git commit -m 'Adiciona MinhaFeature'`)
4. Push para a branch (`git push origin feature/MinhaFeature`)
5. Abra um Pull Request

### Áreas para Contribuir

- [ ] Adicionar mais bancos de dados (Redis, DynamoDB, etc)
- [ ] Implementar refresh tokens
- [ ] Adicionar 2FA (Two-Factor Authentication)
- [ ] Rate limiting
- [ ] Email verification
- [ ] Password reset
- [ ] OAuth2 integration
- [ ] GraphQL support

---

## 📄 Licença

Este projeto está licenciado sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

---

## 🙏 Agradecimentos

- [Axum](https://github.com/tokio-rs/axum) - Framework web
- [SQLx](https://github.com/launchbadge/sqlx) - SQL toolkit
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - JWT implementation
- [argon2](https://github.com/RustCrypto/password-hashes) - Password hashing

---

## 📞 Suporte

Se você tiver problemas ou dúvidas:

1. Verifique a [documentação](#-índice)
2. Procure por [issues existentes](https://github.com/seu-usuario/auth-system-rust/issues)
3. Abra uma [nova issue](https://github.com/seu-usuario/auth-system-rust/issues/new)

---

## 🎓 Aprenda Mais

- [Documentação do Rust](https://doc.rust-lang.org/)
- [Axum Documentation](https://docs.rs/axum/)
- [JWT.io](https://jwt.io/)
- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)

---

<div align="center">

**Feito com ❤️ e Rust 🦀**

[⬆ Voltar ao topo](#-auth-system-rust)

</div>
