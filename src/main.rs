mod database;

use crate::database::{Database, DatabaseConfig};
use serde_json::json;

fn main() {
    let mut db = Database::new(DatabaseConfig {
        path: String::from("db.json"),
    });

    db.insert_one(json!({
        "name": "Billy",
        "age": 27,
    }));

    db.insert_one(json!({
        "name": "Carisa",
        "age": 26,
    }));

    let query = json!({
        "age": 28,
    });

    match db.find_one(query) {
        None => println!("No results"),
        Some(result) => println!("Results: {}", result),
    }
}
