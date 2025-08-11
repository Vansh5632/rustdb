# RustDB Schema System

This document explains the advanced schema system implemented in RustDB.

## Features

### 1. Compile-Time Validation
- Table names are validated at compile time
- Only alphanumeric characters and underscores are allowed
- Empty table names are rejected
- Field count is tracked at compile time

### 2. Runtime Validation
- Custom validation logic can be implemented
- Schema validation is called before database operations
- Validation errors are properly typed and handled

### 3. Macro-Based Schema Definition
- Clean, declarative syntax for defining schemas
- Automatic implementation of required traits
- Support for custom attributes and derives

## Usage Examples

### Basic Schema with Default Validation

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

// Use basic schema implementation (no custom validation)
rust_db::impl_basic_schema!(User, "User");
```

### Schema with Custom Validation

```rust
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

// Access compile-time schema information
println!("Table name: {}", User::TABLE_NAME);
println!("Field count: {}", User::FIELD_COUNT);
println!("Valid schema: {}", User::validate_at_compile_time());
```

## Compile-Time Errors

The schema system will catch invalid table names at compile time:

```rust
// This will fail to compile!
rust_db::schema! {
    table_name: "Invalid-Name!", // Contains invalid characters
    struct BadSchema {
        id: u64,
    }
}
// Error: Table name contains invalid characters
```

## Error Handling

The schema system provides typed errors:

```rust
#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("Index field '{0}' cannot be empty")]
    IndexFieldEmpty(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}
```

## Traits

### Schema Trait
```rust
pub trait Schema {
    fn schema_validate(&self) -> Result<(), SchemaError>;
    fn table_name() -> &'static str;
}
```

### CompileTimeSchema Trait
```rust
pub trait CompileTimeSchema {
    const TABLE_NAME: &'static str;
    const FIELD_COUNT: usize;
    
    fn validate_at_compile_time() -> bool {
        !Self::TABLE_NAME.is_empty() && Self::FIELD_COUNT > 0
    }
}
```

## Benefits

1. **Type Safety**: All schemas are strongly typed
2. **Compile-Time Validation**: Invalid schemas are caught early
3. **Runtime Validation**: Custom business logic validation
4. **Clean API**: Declarative schema definition
5. **Flexibility**: Support for both basic and custom implementations
6. **Integration**: Works seamlessly with Serde for serialization
