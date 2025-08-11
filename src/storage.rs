use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use serde::{Serialize, Deserialize};
use bincode;
use chrono; // Ensure chrono is in Cargo.toml
use crate::error::DbError;

/// WAL operation enum: represents what gets logged
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum StorageOp {
    Insert(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
}

/// Write-Ahead Log
pub struct Wal {
    writer: BufWriter<File>,
}

impl std::fmt::Debug for Wal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wal")
            .field("writer", &"BufWriter<File>")
            .finish()
    }
}

impl Wal {
    pub fn new(path: &Path) -> Result<Self, DbError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Wal {
            writer: BufWriter::new(file),
        })
    }

    pub(crate) fn write(&mut self, op: &StorageOp) -> Result<(), DbError> {
        bincode::serialize_into(&mut self.writer, op)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;
        self.writer.flush()?;
        Ok(())
    }
}

/// In-memory table
#[derive(Debug)]
pub struct MemTable {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
    size: usize,
}

impl MemTable {
    pub fn new() -> Self {
        MemTable {
            data: BTreeMap::new(),
            size: 0,
        }
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.size += key.len() + value.len();
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.data.get(key).cloned()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

/// Main LSM storage engine
#[derive(Debug)]
pub struct LsmStorage {
    memtable: Arc<RwLock<MemTable>>,
    wal: RwLock<Wal>,
    sstables: RwLock<Vec<PathBuf>>,
    flush_threshold: usize,
}

impl LsmStorage {
    pub fn new(path: &Path, flush_threshold: usize) -> Result<Self, DbError> {
        let wal_path = path.join("wal.log");
        let wal = Wal::new(&wal_path)?;

        Ok(LsmStorage {
            memtable: Arc::new(RwLock::new(MemTable::new())),
            wal: RwLock::new(wal),
            sstables: RwLock::new(Vec::new()),
            flush_threshold,
        })
    }

    pub fn insert(&self, key: Vec<u8>, value: Vec<u8>) -> Result<(), DbError> {
        let mut wal = self.wal.write().unwrap();
        wal.write(&StorageOp::Insert(key.clone(), value.clone()))?;

        let mut memtable = self.memtable.write().unwrap();
        memtable.insert(key, value);

        if memtable.size() >= self.flush_threshold {
            drop(memtable); // unlock before flush
            self.flush_memtable()?;
        }

        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let memtable = self.memtable.read().unwrap();
        if let Some(value) = memtable.get(key) {
            return Some(value);
        }

        // SSTable lookup (not implemented here yet)
        None
    }

    fn flush_memtable(&self) -> Result<(), DbError> {
        let mut memtable = self.memtable.write().unwrap();
        let snapshot = std::mem::replace(&mut *memtable, MemTable::new());

        let sstable_name = format!("sst-{}.bin", chrono::Utc::now().timestamp());
        let sstable_path = PathBuf::from(&sstable_name);
        let mut file = File::create(&sstable_path)?;

        for (key, value) in snapshot.data {
            bincode::serialize_into(&mut file, &(key, value))
                .map_err(|e| DbError::SerializationError(e.to_string()))?;
        }

        self.sstables.write().unwrap().push(sstable_path);

        self.wal.write().unwrap().writer.flush()?;

        Ok(())
    }
}
