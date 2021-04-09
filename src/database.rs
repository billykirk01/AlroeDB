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

    pub fn insert_one(&mut self, document: Value) -> Result<(), &'static str> {
        match document.as_object() {
            None => Err("document to insert was invalid"),
            Some(_) => {
                self.documents.push(document);
                self.save();
                Ok(())
            }
        }
    }

    pub fn insert_many(&mut self, documents: Value) -> Result<(), &'static str> {
        match documents.as_array() {
            None => Err("documents to insert were invalid"),
            Some(doc_vec) => {
                self.documents.append(&mut doc_vec.to_owned());
                self.save();
                Ok(())
            }
        }
    }

    pub fn find_one(&self, query: Value) -> Option<Value> {
        match self.search_documents(query) {
            None => None,
            Some(found) => Some(self.documents[found[0]].to_owned()),
        }
    }

    pub fn find_many(&self, query: Value) -> Option<Vec<Value>> {
        match self.search_documents(query) {
            None => None,
            Some(found) => {
                let mut results: Vec<Value> = Vec::new();
                for index in found {
                    results.push(self.documents[index].to_owned())
                }
                Some(results)
            }
        }
    }

    fn search_documents(&self, query: Value) -> Option<Vec<usize>> {
        let mut found: Vec<usize> = Vec::new();

        for (index, document) in self.documents.iter().enumerate() {
            let mut include = true;

            match query.as_object() {
                None => break,
                Some(pairs) => {
                    for (key, query_value) in pairs {
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
            }
        }

        if found.len() == 0 {
            return None;
        }

        Some(found)
    }

    fn match_values(&self, query_value: &Value, document_value: &Value) -> bool {
        if query_value == document_value {
            return true;
        }

        false
    }

    fn save(&self) {
        write(
            &self.config.path,
            to_string_pretty(&self.documents).unwrap(),
        )
        .unwrap();
    }
}
