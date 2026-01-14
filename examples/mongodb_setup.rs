// Initial setup for MongoDB (OPTIONAL)
// This file is not a traditional "migration", as MongoDB is schema-less.
// It only creates indexes to improve search performance.
//
// How to use:
// 1. Configure MONGODB_URI and MONGODB_DATABASE in .env
// 2. Run: cargo run --example mongodb_setup

use mongodb::{Client, options::IndexOptions, IndexModel};
use mongodb::bson::doc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let mongodb_uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let mongodb_database = std::env::var("MONGODB_DATABASE")
        .unwrap_or_else(|_| "auth_db".to_string());
    
    println!("🍃 Connecting to MongoDB at: {}", mongodb_uri);
    
    let client = Client::with_uri_str(&mongodb_uri).await?;
    let db = client.database(&mongodb_database);
    let collection = db.collection::<mongodb::bson::Document>("users");
    
    println!("📊 Creating indexes on 'users' collection...");
    
    // Unique index for email
    let email_index = IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();
    
    // Unique index for username
    let username_index = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();
    
    // Index for created_at (useful for sorting)
    let created_at_index = IndexModel::builder()
        .keys(doc! { "created_at": -1 })
        .build();
    
    // Create all indexes
    collection.create_indexes(vec![
        email_index,
        username_index,
        created_at_index,
    ]).await?;
    
    println!("✅ Indexes created successfully!");
    println!("✅ MongoDB is ready to use.");
    println!("\nCreated indexes:");
    println!("  - email (unique)");
    println!("  - username (unique)");
    println!("  - created_at (descending)");
    
    Ok(())
}
