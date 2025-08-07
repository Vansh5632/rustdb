use thiserror::Error;

#[derive(Error,Debug)]
pub enum DbError{
    #[error("Schema validation error: {0}")]
    SchemaError(String),

    #[error("Storage error: {0}")]
    StorageError(#[from] std::io::Error),

    #[error("Serialization error :{0}")]
    SerializationError(String),
}

#[derive(Error,Debug)]
pub enum SchemaError{
    #[error("Index field '{0}' cannot be empty")]
    IndexFieldEmpty(String)
}