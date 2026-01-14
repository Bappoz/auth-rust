# 📝 MongoDB Setup (Opcional)

O MongoDB **não precisa de migrations** porque é schema-less (sem schema fixo).

## Por que não há migrations MongoDB?

- ✅ MongoDB cria collections automaticamente
- ✅ Não precisa definir estrutura antes
- ✅ Cada documento pode ter campos diferentes
- ✅ Muito mais flexível que SQL

## Como configurar MongoDB?

### Opção 1: Deixar automático (recomendado)

Simplesmente inicie sua aplicação com MongoDB configurado. A collection `users` será criada automaticamente no primeiro registro de usuário.

```bash
# Configure o .env
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=auth_db

# Execute a aplicação
cargo run --features mongodb
```

### Opção 2: Criar índices manualmente (melhor performance)

Para criar índices que melhoram a performance:

```bash
# Execute o script de setup
cargo run --example mongodb_setup --features mongodb
```

Ou crie manualmente no MongoDB shell:

```javascript
use auth_db

db.users.createIndex({ "email": 1 }, { unique: true })
db.users.createIndex({ "username": 1 }, { unique: true })
db.users.createIndex({ "created_at": -1 })
```

## Estrutura de Documento

```json
{
  "_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "joao",
  "email": "joao@email.com",
  "password_hash": "$argon2id$v=19$m=19456...",
  "created_at": "2026-01-14T10:30:00Z",
  "updated_at": "2026-01-14T10:30:00Z",
  "is_active": true
}
```

## Índices (Opcional mas Recomendado)

- **email** (unique) - Garante emails únicos e acelera buscas
- **username** (unique) - Garante usernames únicos e acelera buscas
- **created_at** (descending) - Acelera ordenação por data

---

## 📊 Comparação: SQL vs NoSQL

| Aspecto             | SQL (Postgres/MySQL)     | NoSQL (MongoDB)           |
| ------------------- | ------------------------ | ------------------------- |
| Schema              | Rígido (precisa definir) | Flexível (sem schema)     |
| Migrations          | **Obrigatório**          | **Opcional**              |
| Tabelas/Collections | Criar antes              | Cria automático           |
| Índices             | Criar nas migrations     | Criar no código ou manual |
| Constraints         | UNIQUE, NOT NULL, etc    | Validação na aplicação    |
| Mudanças            | Requer migration         | Só atualizar código       |

---

**Conclusão:** Está correto ter apenas migrations SQL! MongoDB não precisa. 🎯
