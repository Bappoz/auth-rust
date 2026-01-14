# 📝 MongoDB Setup (Optional)

MongoDB **does not need migrations** because it is schema-less (no fixed schema).

## Why are there no MongoDB migrations?

- ✅ MongoDB creates collections automatically
- ✅ No need to define structure beforehand
- ✅ Each document can have different fields
- ✅ Much more flexible than SQL

## How to configure MongoDB?

### Option 1: Let it be automatic (recommended)

Simply start your application with MongoDB configured. The `users` collection will be created automatically on the first user registration.

```bash
# Configure .env
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=auth_db

# Run the application
cargo run --features mongodb
```

### Option 2: Create indexes manually (better performance)

To create indexes that improve performance:

```bash
# Run the setup script
cargo run --example mongodb_setup --features mongodb
```

Or create manually in MongoDB shell:

```javascript
use auth_db

db.users.createIndex({ "email": 1 }, { unique: true })
db.users.createIndex({ "username": 1 }, { unique: true })
db.users.createIndex({ "created_at": -1 })
```

## Document Structure

```json
{
  "_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john",
  "email": "john@email.com",
  "password_hash": "$argon2id$v=19$m=19456...",
  "created_at": "2026-01-14T10:30:00Z",
  "updated_at": "2026-01-14T10:30:00Z",
  "is_active": true
}
```

## Indexes (Optional but Recommended)

- **email** (unique) - Ensures unique emails and speeds up searches
- **username** (unique) - Ensures unique usernames and speeds up searches
- **created_at** (descending) - Speeds up sorting by date

---

## 📊 Comparison: SQL vs NoSQL

| Aspect             | SQL (Postgres/MySQL)  | NoSQL (MongoDB)           |
| ------------------ | --------------------- | ------------------------- |
| Schema             | Rigid (must define)   | Flexible (schema-less)    |
| Migrations         | **Required**          | **Optional**              |
| Tables/Collections | Create beforehand     | Creates automatically     |
| Indexes            | Create in migrations  | Create in code or manual  |
| Constraints        | UNIQUE, NOT NULL, etc | Validation in application |
| Changes            | Requires migration    | Just update code          |

---

**Conclusion:** It's correct to have only SQL migrations! MongoDB doesn't need them. 🎯
