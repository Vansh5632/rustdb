mod error;
pub mod schema;
mod storage;

pub use error::{DbError, SchemaError};
pub use schema::{Schema, CompileTimeSchema};
use storage::LsmStorage;
use std::path::Path;
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Database {
    storage: RwLock<LsmStorage>,
}

impl Database {
    pub async fn open(path: &str) -> Result<Self, DbError> {
        let storage = LsmStorage::new(Path::new(path), 1024 * 1024)?; // 1MB flush threshold
        Ok(Database {
            storage: RwLock::new(storage),
        })
    }

    pub async fn insert<T>(&self, item: &T) -> Result<(), DbError>
    where
        T: Schema + Serialize,
    {
        // Schema validation
        item.schema_validate().map_err(|e| DbError::SchemaError(e.to_string()))?;
        
        // Serialize
        let key = T::table_name().as_bytes().to_vec();
        let value = bincode::serialize(item)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;
        
        // Store
        self.storage.write().await.insert(key, value)?;
        Ok(())
    }

    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, DbError>
    where
        T: Schema + DeserializeOwned,
    {
        let key_bytes = key.as_bytes();
        if let Some(data) = self.storage.write().await.get(key_bytes) {
            let item = bincode::deserialize::<T>(&data)
                .map_err(|e| DbError::SerializationError(e.to_string()))?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    pub fn query<T>(&self) -> QueryBuilder<T>
    where
        T: Schema + DeserializeOwned + Send + Sync,
    {
        QueryBuilder::new(self)
    }
}

// Example query builder
pub struct QueryBuilder<'a, T> {
    db: &'a Database,
    filters: Vec<Box<dyn Fn(&T) -> bool + Send + Sync>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> QueryBuilder<'a, T>
where
    T: Schema + DeserializeOwned + Send + Sync,
{
    pub fn new(db: &'a Database) -> Self {
        QueryBuilder {
            db,
            filters: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&T) -> bool + 'static + Send + Sync,
    {
        self.filters.push(Box::new(filter));
        self
    }

    pub async fn execute(self) -> Result<Vec<T>, DbError> {
        // Simplified: Scan all items (in real DB would use indexes)
        let mut results = Vec::new();
        let table_name = T::table_name();
        
        // This is a placeholder - real implementation would iterate properly
        if let Some(item) = self.db.get::<T>(table_name).await? {
            if self.filters.iter().all(|f| f(&item)) {
                results.push(item);
            }
        }
        
        Ok(results)
    }
}