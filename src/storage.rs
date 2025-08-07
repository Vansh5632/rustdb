//LSM tree core 
use std::collections::BTreeMap;
use std::fs::{File,OpenOptions};
use std::io::{self,BufWriter,Write,Read};
use std::path::Path;
use std::sync::{Arc, RwLock};
use memmap::MmapMut;
use serde::{Serialize,Deserialize};
use bincode;
use crossbeam::epoch;
use log::{info,error};
use crate::error::DbError;

#[derive(Debug,Serialize,Deserialize)]
enum StorageOp{
    Insert(Vec<u8>,Vec<u8>),
    Delete(Vec<u8>),
}

impl Wal{
    pub fn new(path: &Path) -> Result<Self,DbError> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Wal{
            writer: BufWriter::new(file),
        })
    }

    pub fn write(&mut self,op: &StorageOp)-> Result<(),DbError>{
        bincode::serialize_into(&mut self.writer,op).map_err(|e| DbError::SerializationError(e.to_string()))?;
        self.writer.flush()?;
        Ok(())
    }
}

pub struct MemTable{
    data:BTreeMap<Vec<u8>,Vec<u8>>,
    size:usize,
}

impl MemTable{
    pub fn new() ->Self{
        MemTable{
            data:BTreeMap::new(),
            size:0,
        }

    }
    pub fn insert(&mut self,key:Vec<u8>,value:Vec<u8>){
        self.size+= key.len()+ value.len();
        self.data.insert(key,value);
    }

    pub fn get(&self, key:&[u8])->Option<Vec<u8>>{
        self.data.get(key).cloned()
    }

}

pub struct LsmStorage{
    memtable:Arc<RwLock<MemTable>>,
    wal:RwLock<Wal>,
    sstables:RwLock<Vec<String>>,
    flush_threshold:usize,
}

impl LsmStorage{
    pub fn new(path: &Path,flush_threshold:usize)-> Result<Self,DbError>{
        let wal_path = path.join("wal.log");
        let wal =Wal::new(&wal_path)?;

        Ok(LsmStorage{
            memtable:Arc::new(RwLock::new(MemTable::new())),
            wal:RwLock::new(wal),
            sstables:RwLock::new(Vec::new()),
            flush_threshold,
        })
    }

    pub fn get(&self,key: &[u8])-> Option<Vec<u8>> {
        let memtable = self.memtable.read().unwrap();
        if let Some(value)= memtable.get(key){
            return Some(value);
        }

        None
    }

    fn flush_memtable(&self)->Result<(),DbError>{
        let mut memtable = self.memtable.write().unwrap();
        let snapshot = std::mem::replace(&mut *memtable,MemTable::new());

        let sstable_name = format!("sst-{}.bin", chrono::Utc::now().timestamp());
        let mut file = File::create(&sstable_name)?;

        for(key,value) in snapshot.data{
            bincode::serialize_into(&mut file,&(key,value)).map_err(|e| DbError::SerializationError(e.to_string()))?;
        }

        self.sstables.write().unwrap().push(sstable_name);

        self.wal.write()?.writer.flush()?;

        Ok(())
    }
}