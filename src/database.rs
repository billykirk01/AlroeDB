use serde_json::{to_string_pretty, Value};
use std::fs::write;
use std::vec::Vec;

pub struct Database {
    config: DatabaseConfig,
    documents: Vec<Value>,
}

pub struct DatabaseConfig {
    pub path: String,
}

impl Database {
    pub fn new(config: DatabaseConfig) -> Database {
        Database {
            config: config,
            documents: Vec::new(),
        }
    }

    pub fn insert_one(&mut self, document: Value) {
        self.documents.push(document);
        self.save()
    }

    pub fn find_one(&self, query: Value) -> Option<Value> {
        let found = self.search_documents(query);
        if found.len() == 0 {
            return None;
        }
        Some(self.documents[found[0]].to_owned())
    }

    fn search_documents(&self, query: Value) -> Vec<usize> {
        let mut found: Vec<usize> = Vec::new();

        for (index, document) in self.documents.iter().enumerate() {
            let mut include = true;

            for (key, query_value) in query.as_object().unwrap() {
                if !include {
                    break;
                }

                let document_value = &document[key];
                include = self.match_values(query_value, document_value);

                if include {
                    found.push(index);
                }
            }
        }

        found
    }

    fn match_values(&self, query_value: &Value, document_value: &Value) -> bool {
        if query_value == document_value {
            return true;
        }
        return false;
    }

    fn save(&self) {
        let string = to_string_pretty(&self.documents).unwrap();
        write(&self.config.path, string).unwrap();
    }
}
