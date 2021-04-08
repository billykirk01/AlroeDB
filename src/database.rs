use serde_json::{to_string_pretty, Value};
use std::fs::write;
use std::sync::Mutex;
use std::vec::Vec;

pub struct Database {
    config: DatabaseConfig,
    documents: Vec<Value>,
    lock: Mutex<()>,
}

pub struct DatabaseConfig {
    pub path: String,
}

impl Database {
    pub fn new(config: DatabaseConfig) -> Database {
        Database {
            config: config,
            documents: Vec::new(),
            lock: Mutex::new(()),
        }
    }

    pub fn insert_one(&mut self, document: Value) {
        let _lock = self.lock.lock().unwrap();
        self.documents.push(document);
        self.save()
    }

    fn save(&self) {
        let string = to_string_pretty(&self.documents).unwrap();
        write(&self.config.path, string).unwrap();
    }
}
