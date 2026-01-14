// Setup inicial para MongoDB (OPCIONAL)
// Este arquivo não é uma "migration" tradicional, pois MongoDB é schema-less.
// Ele apenas cria índices para melhorar a performance das buscas.
//
// Como usar:
// 1. Configure MONGODB_URI e MONGODB_DATABASE no .env
// 2. Execute: cargo run --example mongodb_setup

use mongodb::{Client, options::IndexOptions, IndexModel};
use mongodb::bson::doc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let mongodb_uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let mongodb_database = std::env::var("MONGODB_DATABASE")
        .unwrap_or_else(|_| "auth_db".to_string());
    
    println!("🍃 Conectando ao MongoDB em: {}", mongodb_uri);
    
    let client = Client::with_uri_str(&mongodb_uri).await?;
    let db = client.database(&mongodb_database);
    let collection = db.collection::<mongodb::bson::Document>("users");
    
    println!("📊 Criando índices na collection 'users'...");
    
    // Índice único para email
    let email_index = IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();
    
    // Índice único para username
    let username_index = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();
    
    // Índice para created_at (útil para ordenação)
    let created_at_index = IndexModel::builder()
        .keys(doc! { "created_at": -1 })
        .build();
    
    // Cria todos os índices
    collection.create_indexes(vec![
        email_index,
        username_index,
        created_at_index,
    ]).await?;
    
    println!("✅ Índices criados com sucesso!");
    println!("✅ MongoDB está pronto para uso.");
    println!("\nÍndices criados:");
    println!("  - email (unique)");
    println!("  - username (unique)");
    println!("  - created_at (descending)");
    
    Ok(())
}
