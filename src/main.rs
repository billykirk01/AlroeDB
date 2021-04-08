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
}
