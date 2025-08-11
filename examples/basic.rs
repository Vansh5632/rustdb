use rust_db::{Database, DbError};
use serde::{Serialize, Deserialize};

// Using the new schema macro with compile-time validation
rust_db::schema! {
    table_name: "User",
    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct User {
        id: u64,
        name: String,
        email: String,
        age: u32,
    }
}

// Custom validation implementation
impl User {
    fn validate_email(&self) -> bool {
        self.email.contains('@') && self.email.contains('.')
    }
}

// Override the default schema validation
impl rust_db::Schema for User {
    fn schema_validate(&self) -> Result<(), rust_db::SchemaError> {
        if self.name.is_empty() {
            return Err(rust_db::SchemaError::ValidationError("Name cannot be empty".to_string()));
        }
        if !self.validate_email() {
            return Err(rust_db::SchemaError::ValidationError("Invalid email format".to_string()));
        }
        if self.age > 150 {
            return Err(rust_db::SchemaError::ValidationError("Age seems unrealistic".to_string()));
        }
        Ok(())
    }

    fn table_name() -> &'static str {
        "User"
    }
}

#[tokio::main]
async fn main() -> Result<(), DbError> {
    pretty_env_logger::init();
    let db = Database::open("./data").await?;

    // Test with valid user
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
    };

    db.insert(&user).await?;
    println!("✅ Inserted valid user: {:?}", user);

    // Test compile-time schema validation
    use rust_db::CompileTimeSchema;
    println!("📊 Schema info:");
    println!("  Table name: {}", User::TABLE_NAME);
    println!("  Field count: {}", User::FIELD_COUNT);
    println!("  Compile-time validation: {}", User::validate_at_compile_time());

    // Test query functionality
    let results = db.query::<User>().filter(|u| u.age > 25).execute().await?;
    println!("👥 Users over 25: {:?}", results);

    // Test validation errors
    println!("\n🚨 Testing validation errors:");
    
    // Test empty name
    let invalid_user = User {
        id: 2,
        name: "".to_string(),
        email: "test@example.com".to_string(),
        age: 25,
    };
    
    match db.insert(&invalid_user).await {
        Err(e) => println!("❌ Expected error for empty name: {}", e),
        Ok(_) => println!("❌ Should have failed for empty name"),
    }

    // Test invalid email
    let invalid_user = User {
        id: 3,
        name: "Bob".to_string(),
        email: "invalid-email".to_string(),
        age: 25,
    };
    
    match db.insert(&invalid_user).await {
        Err(e) => println!("❌ Expected error for invalid email: {}", e),
        Ok(_) => println!("❌ Should have failed for invalid email"),
    }

    // Test unrealistic age
    let invalid_user = User {
        id: 4,
        name: "Charlie".to_string(),
        email: "charlie@example.com".to_string(),
        age: 200,
    };
    
    match db.insert(&invalid_user).await {
        Err(e) => println!("❌ Expected error for unrealistic age: {}", e),
        Ok(_) => println!("❌ Should have failed for unrealistic age"),
    }

    Ok(())
}