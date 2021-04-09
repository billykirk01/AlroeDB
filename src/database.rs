use serde_json;
use std::collections::HashMap;
use std::fs::write;
use std::vec::Vec;

pub struct Database {
    config: DatabaseConfig,
    documents: Vec<serde_json::Value>,
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

    pub fn insert_one(&mut self, document: serde_json::Value) -> Result<(), &'static str> {
        match document.as_object() {
            None => Err("document to insert was invalid"),
            Some(_) => {
                self.documents.push(document);
                self.save();
                Ok(())
            }
        }
    }

    pub fn insert_many(&mut self, documents: serde_json::Value) -> Result<(), &'static str> {
        match documents.as_array() {
            None => Err("documents to insert were invalid"),
            Some(doc_vec) => {
                self.documents.append(&mut doc_vec.to_owned());
                self.save();
                Ok(())
            }
        }
    }

    pub fn find_one(&self, query: serde_json::Value) -> Option<serde_json::Value> {
        match self.search_documents(query) {
            None => None,
            Some(found) => Some(self.documents[found[0]].to_owned()),
        }
    }

    pub fn find_many(&self, query: serde_json::Value) -> Option<Vec<serde_json::Value>> {
        match self.search_documents(query) {
            None => None,
            Some(found) => {
                let mut results: Vec<serde_json::Value> = Vec::new();
                for index in found.into_iter() {
                    results.push(self.documents[index].to_owned())
                }
                Some(results)
            }
        }
    }

    pub fn update_one(&mut self, query: serde_json::Value) -> Result<(), &'static str> {
        match query.as_object() {
            None => Err("document to insert was invalid"),
            Some(_) => {
                todo!();
            }
        }
    }

    pub fn update_many(&mut self, query: serde_json::Value) -> Result<(), &'static str> {
        match query.as_array() {
            None => Err("documents to insert were invalid"),
            Some(doc_vec) => {
                todo!();
            }
        }
    }

    pub fn delete_one(&mut self, query: serde_json::Value) -> Result<(), &'static str> {
        match query.as_object() {
            None => Err("query was invalid"),
            Some(_) => match self.search_documents(query) {
                None => Err("no documents found matching query"),
                Some(found) => {
                    self.documents.remove(found[0]);
                    self.save();
                    Ok(())
                }
            },
        }
    }

    pub fn delete_many(&mut self, query: serde_json::Value) -> Result<(), &'static str> {
        match query.as_object() {
            None => Err("query was invalid"),
            Some(_) => match self.search_documents(query) {
                None => Err("no documents found matching query"),
                Some(found) => {
                    let mut found_map: HashMap<usize, bool> = HashMap::new();
                    for index in found.into_iter() {
                        found_map.insert(index, true);
                    }
                    let mut temp = Vec::new();
                    for (index, document) in self.documents.iter().enumerate() {
                        if !found_map.contains_key(&index) {
                            temp.push(document.to_owned())
                        }
                    }
                    self.documents = temp;
                    self.save();
                    Ok(())
                }
            },
        }
    }

    fn search_documents(&self, query: serde_json::Value) -> Option<Vec<usize>> {
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

    fn match_values(
        &self,
        query_value: &serde_json::Value,
        document_value: &serde_json::Value,
    ) -> bool {
        if query_value == document_value {
            return true;
        }

        false
    }

    fn save(&self) {
        write(
            &self.config.path,
            serde_json::to_string_pretty(&self.documents).unwrap(),
        )
        .unwrap();
    }
}
