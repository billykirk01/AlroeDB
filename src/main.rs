mod database;

use crate::database::{Database, DatabaseConfig};
use serde_json::json;

fn main() {
    let mut db = Database::new(DatabaseConfig {
        path: "db.json".to_string(),
    });

    let success = db.insert_one(json!({
        "name": "Billy",
        "age": 27,
    }));

    match success {
        Err(e) => println!("{}", e),
        Ok(()) => println!("Successfully added document"),
    }

    let success = db.insert_many(json!([{
        "name": "Tanner",
        "age": 27,
    },{
        "name": "Carisa",
        "age": 26,
    }]));

    match success {
        Err(e) => println!("{}", e),
        Ok(()) => println!("Successfully added documents"),
    }

    let query = json!({
        "name": "Billy",
    });

    match db.find_one(query) {
        None => println!("No results"),
        Some(result) => println!("Results: {}", result),
    }

    let query = json!({
        "age": 27,
    });

    match db.find_many(query) {
        None => println!("No results"),
        Some(results) => println!("Results: {:?}", results),
    }

    let query = json!({
        "age": 27,
    });

    match db.delete_many(query) {
        Err(e) => println!("{}", e),
        Ok(()) => println!("Successfully deleted documents"),
    }

    let query = json!({
        "name": "Carisa",
    });

    match db.delete_one(query) {
        Err(e) => println!("{}", e),
        Ok(()) => println!("Successfully deleted document"),
    }
}
