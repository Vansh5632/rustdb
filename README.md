# RustDB 🦀

A high-performance, type-safe embedded database written in Rust with advanced schema validation and compile-time safety guarantees.

## 🌟 What We're Building

RustDB is a modern embedded database designed for Rust applications that prioritizes:

- **Type Safety**: Leverage Rust's type system for database schemas
- **Compile-Time Validation**: Catch schema errors before your code even runs
- **Performance**: Built on LSM-tree storage architecture for fast writes
- **Simplicity**: Clean, intuitive API with macro-based schema definition
- **Async/Await Support**: Fully async database operations using Tokio

## 🏗️ Architecture

### Core Components

1. **Schema System**: Advanced macro-based schema definition with compile-time validation
2. **Storage Engine**: LSM-tree based storage with Write-Ahead Logging (WAL)
3. **Query Builder**: Functional query interface with filtering capabilities
4. **Type Safety**: Full integration with Serde for serialization/deserialization

### Storage Architecture

- **MemTable**: In-memory write buffer for fast insertions
- **WAL (Write-Ahead Log)**: Ensures durability and crash recovery
- **SSTable**: Immutable sorted files for efficient reads
- **Compaction**: Background process to optimize storage (planned)

## 🚀 Features

### ✅ Implemented

- [x] **Compile-Time Schema Validation**
  - Table name validation (alphanumeric + underscore only)
  - Field count tracking
  - Type safety guarantees

- [x] **Runtime Schema Validation**
  - Custom validation logic
  - Typed error handling
  - Business rule enforcement

- [x] **LSM Storage Engine**
  - In-memory write buffer
  - Write-ahead logging
  - Basic persistence

- [x] **Query System**
  - Functional query builder
  - Filter chaining
  - Type-safe operations

- [x] **Async Operations**
  - Full Tokio integration
  - Non-blocking I/O
  - Concurrent access support

### 🔄 In Progress

- [ ] **Advanced Indexing**
  - B-tree indexes
  - Composite indexes
  - Index optimization

- [ ] **SSTable Implementation**
  - Efficient range queries
  - Compression support
  - Bloom filters

- [ ] **Query Optimization**
  - Query planner
  - Index utilization
  - Cost-based optimization

### 🎯 Planned

- [ ] **Transactions**
  - ACID compliance
  - Optimistic concurrency control
  - Deadlock detection

- [ ] **Distributed Features**
  - Replication support
  - Consensus algorithms
  - Horizontal scaling

- [ ] **Advanced Query Language**
  - SQL-like syntax
  - Aggregation functions
  - Join operations

## 🔧 Installation

Add RustDB to your `Cargo.toml`:

```toml
[dependencies]
rust_db = { path = "." }  # or version when published
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

## 📚 Usage Examples

### Basic Schema Definition

```rust
use rust_db::{Database, DbError};
use serde::{Serialize, Deserialize};

// Define schema with compile-time validation
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

// Use basic schema implementation
rust_db::impl_basic_schema!(User, "User");

#[tokio::main]
async fn main() -> Result<(), DbError> {
    let db = Database::open("./data").await?;
    
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
    };
    
    // Insert data
    db.insert(&user).await?;
    
    // Query with filtering
    let adults = db
        .query::<User>()
        .filter(|u| u.age >= 18)
        .execute()
        .await?;
    
    println!("Adults: {:?}", adults);
    Ok(())
}
```

### Custom Validation

```rust
// Override default validation with custom logic
impl rust_db::Schema for User {
    fn schema_validate(&self) -> Result<(), rust_db::SchemaError> {
        if self.name.is_empty() {
            return Err(rust_db::SchemaError::ValidationError(
                "Name cannot be empty".to_string()
            ));
        }
        if !self.email.contains('@') {
            return Err(rust_db::SchemaError::ValidationError(
                "Invalid email format".to_string()
            ));
        }
        Ok(())
    }

    fn table_name() -> &'static str {
        "User"
    }
}
```

### Compile-Time Schema Information

```rust
use rust_db::CompileTimeSchema;

// Access schema metadata at compile time
println!("Table: {}", User::TABLE_NAME);
println!("Fields: {}", User::FIELD_COUNT);
println!("Valid: {}", User::validate_at_compile_time());
```

## 🛡️ Type Safety

RustDB catches errors at compile time:

```rust
// This will fail to compile! ❌
rust_db::schema! {
    table_name: "Invalid-Name!", // Contains invalid characters
    struct BadSchema {
        id: u64,
    }
}
// Compile error: "Table name contains invalid characters"
```

## 🏃‍♂️ Performance

RustDB is designed for high performance:

- **Write Optimization**: LSM-tree architecture provides excellent write performance
- **Memory Efficiency**: Minimal allocations and zero-copy operations where possible
- **Async I/O**: Non-blocking operations for maximum throughput
- **Type Safety**: Zero-runtime-cost abstractions

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific example
cargo run --example basic
cargo run --example product_schema

# Test compile-time validation (should fail)
cargo run --example invalid_schema
```

## 📁 Project Structure

```
rust_db/
├── src/
│   ├── lib.rs          # Main library interface
│   ├── schema.rs       # Schema system and macros
│   ├── storage.rs      # LSM storage implementation
│   └── error.rs        # Error types
├── examples/
│   ├── basic.rs        # Basic usage example
│   ├── product_schema.rs # Product schema example
│   └── invalid_schema.rs # Compile-time validation demo
├── SCHEMA.md           # Detailed schema documentation
└── README.md           # This file
```

## 🎯 Design Goals

1. **Safety First**: Leverage Rust's type system to prevent runtime errors
2. **Performance**: Match or exceed performance of established databases
3. **Developer Experience**: Clean, intuitive API with excellent error messages
4. **Reliability**: Robust error handling and data consistency guarantees
5. **Flexibility**: Support for various use cases from embedded to distributed

## 🤝 Contributing

We welcome contributions! Areas where help is needed:

- **Performance Optimization**: Benchmarking and optimization
- **Feature Implementation**: See the roadmap above
- **Documentation**: Examples, tutorials, and API docs
- **Testing**: Unit tests, integration tests, and fuzzing

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

- [RocksDB](https://rocksdb.org/) - High-performance key-value store
- [SQLite](https://sqlite.org/) - Embedded SQL database
- [Sled](https://github.com/spacejam/sled) - Rust embedded database
- [TiKV](https://tikv.org/) - Distributed key-value database in Rust

---

**Note**: RustDB is currently in active development. APIs may change before the 1.0 release. Use in production environments at your own risk.

Built with ❤️ and 🦀 by the Rust community.
