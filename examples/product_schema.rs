use rust_db::{Database, DbError, CompileTimeSchema};
use serde::{Serialize, Deserialize};

// Define a simple Product schema using the macro
rust_db::schema! {
    table_name: "Product",
    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct Product {
        id: u64,
        name: String,
        price: f64,
        category: String,
    }
}

// Use the basic schema implementation
rust_db::impl_basic_schema!(Product, "Product");

#[tokio::main]
async fn main() -> Result<(), DbError> {
    pretty_env_logger::init();
    let db = Database::open("./data").await?;

    println!("🛍️  Product Schema Demo");
    println!("Table name: {}", Product::TABLE_NAME);
    println!("Field count: {}", Product::FIELD_COUNT);

    let product = Product {
        id: 1,
        name: "Laptop".to_string(),
        price: 999.99,
        category: "Electronics".to_string(),
    };

    db.insert(&product).await?;
    println!("✅ Inserted product: {:?}", product);

    let expensive_products = db
        .query::<Product>()
        .filter(|p| p.price > 500.0)
        .execute()
        .await?;
    
    println!("💰 Expensive products: {:?}", expensive_products);

    Ok(())
}
