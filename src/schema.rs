// Define the Schema trait
pub trait Schema {
    fn schema_validate(&self) -> Result<(), crate::SchemaError>;
    fn table_name() -> &'static str;
}
// Macro to derive Schema implementation with compile-time validation
#[macro_export]
macro_rules! schema {
    (
        table_name: $table:literal,
        $(#[$attr:meta])*
        struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_type:ty $(,)?
            )*
        }
    ) => {
        $(#[$attr])*
        pub struct $name {
            $(
                $(#[$field_attr])*
                pub $field: $field_type,
            )*
        }

        impl $crate::CompileTimeSchema for $name {
            const TABLE_NAME: &'static str = $table;
            const FIELD_COUNT: usize = {
                let mut count = 0;
                $(
                    let _ = stringify!($field);
                    count += 1;
                )*
                count
            };
        }

        // Compile-time validation
        const _: () = {
            // Ensure table name is not empty
            const TABLE_NAME: &str = $table;
            if TABLE_NAME.is_empty() {
                panic!("Table name cannot be empty");
            }
            
            // Ensure table name contains only valid characters
            let bytes = TABLE_NAME.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                let byte = bytes[i];
                if !((byte >= b'a' && byte <= b'z') || 
                     (byte >= b'A' && byte <= b'Z') || 
                     (byte >= b'0' && byte <= b'9') || 
                     byte == b'_') {
                    panic!("Table name contains invalid characters");
                }
                i += 1;
            }
        };
    };
}

// Helper macro for basic Schema implementation (can be overridden)
#[macro_export]
macro_rules! impl_basic_schema {
    ($name:ident, $table:literal) => {
        impl $crate::Schema for $name {
            fn schema_validate(&self) -> Result<(), $crate::SchemaError> {
                // Basic validation - can be overridden
                Ok(())
            }

            fn table_name() -> &'static str {
                $table
            }
        }
    };
}

// Compile-time schema validator trait
pub trait CompileTimeSchema {
    const TABLE_NAME: &'static str;
    const FIELD_COUNT: usize;
    
    fn validate_at_compile_time() -> bool {
        !Self::TABLE_NAME.is_empty() && Self::FIELD_COUNT > 0
    }
}