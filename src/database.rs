use serde_json::{Map, Value};
use std::collections::HashMap;
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

    pub fn insert_one(&mut self, document: Value) -> Result<(), String> {
        match document.as_object() {
            None => return Err("documents to insert were invalid".to_string()),
            Some(_) => self.documents.push(document),
        };

        self.save()
    }

    pub fn insert_many(&mut self, documents: Value) -> Result<(), String> {
        match documents.as_array() {
            None => return Err("documents to insert were invalid".to_string()),
            Some(docs) => self.documents.append(&mut docs.to_owned()),
        };

        self.save()
    }

    pub fn find_one(&self, query: Value) -> Option<Value> {
        let query = query.as_object()?;

        match self.search_documents(query) {
            None => None,
            Some(found) => Some(self.documents[found[0]].to_owned()),
        }
    }

    pub fn find_many(&self, query: Value) -> Option<Vec<Value>> {
        let query = query.as_object()?;

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

    pub fn update_one(&mut self, query: Value, update: Value) -> Result<(), String> {
        let query = match query.as_object() {
            None => return Err("query was invalid".to_string()),
            Some(query) => query,
        };

        let update = match update.as_object() {
            None => return Err("updates were invalid".to_string()),
            Some(update) => update,
        };

        let found = match self.search_documents(query) {
            None => return Err("document to delete not found".to_string()),
            Some(found) => found,
        };

        match self.documents[found[0]].as_object() {
            None => return Err("document to delete not found".to_string()),
            Some(doc) => {
                let mut temp = doc.to_owned();
                for (key, value) in update.iter() {
                    temp.insert(key.to_owned(), value.to_owned());
                }
                self.documents[found[0]] = Value::Object(temp);
            }
        }

        self.save()
    }

    pub fn update_many(&mut self, query: Value, update: Value) -> Result<(), String> {
        let query = match query.as_object() {
            None => return Err("query was invalid".to_string()),
            Some(query) => query,
        };

        let update = match update.as_object() {
            None => return Err("updates were invalid".to_string()),
            Some(update) => update,
        };

        let found_map = match self.search_documents(query) {
            None => return Err("document to delete not found".to_string()),
            Some(found) => {
                let mut found_map: HashMap<usize, bool> = HashMap::new();
                for index in found.into_iter() {
                    found_map.insert(index, true);
                }
                found_map
            }
        };

        for (index, _) in found_map {
            match self.documents[index].as_object() {
                None => return Err("document to delete not found".to_string()),
                Some(doc) => {
                    let mut temp = doc.to_owned();
                    for (key, value) in update.iter() {
                        temp.insert(key.to_owned(), value.to_owned());
                    }
                    self.documents[index] = Value::Object(temp);
                }
            }
        }

        self.save()
    }

    pub fn delete_one(&mut self, query: Value) -> Result<(), String> {
        let query = match query.as_object() {
            None => return Err("query was invalid".to_string()),
            Some(query) => query,
        };

        match self.search_documents(query) {
            None => return Err("document to delete not found".to_string()),
            Some(found) => {
                self.documents.remove(found[0]);
            }
        };

        self.save()
    }

    pub fn delete_many(&mut self, query: Value) -> Result<(), String> {
        let query = match query.as_object() {
            None => return Err("query was invalid".to_string()),
            Some(query) => query,
        };

        match self.search_documents(query) {
            None => return Err("document to delete not found".to_string()),
            Some(found) => {
                let mut found_map: HashMap<usize, bool> = HashMap::new();
                for index in found.into_iter() {
                    found_map.insert(index, true);
                }
                let mut temp_documents = Vec::new();
                for (index, document) in self.documents.iter().enumerate() {
                    if !found_map.contains_key(&index) {
                        temp_documents.push(document.to_owned())
                    }
                }
                self.documents = temp_documents;
            }
        };

        self.save()
    }

    fn search_documents(&self, query: &Map<String, Value>) -> Option<Vec<usize>> {
        let mut found: Vec<usize> = Vec::new();

        for (index, document) in self.documents.iter().enumerate() {
            let mut include = true;

            for (query_key, query_value) in query {
                if !include {
                    break;
                }

                let document_value = &document[query_key];

                include = self.match_values(query_value, document_value);

                if include {
                    found.push(index);
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

    fn save(&self) -> Result<(), String> {
        match write(
            &self.config.path,
            serde_json::to_string_pretty(&self.documents).unwrap(),
        ) {
            Err(e) => Err(e.to_string()),
            Ok(()) => Ok(()),
        }
    }
}
