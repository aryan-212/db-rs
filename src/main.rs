use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, http::header, web};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // CRUD Operations
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    fn get_by_username(&self, name: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == name)
    }

    // Save data to file
    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(&self)?; // Use pretty formatting for readability
        let mut file = File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // Load data from file
    fn load_from_file() -> std::io::Result<Self> {
        let mut file = File::open("database.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let db: Database = serde_json::from_str(&contents)?;
        Ok(db)
    }
}

fn main() {
    println!("Hello, World!");

    // Example usage of Database
    let mut db = Database::new();
    db.insert(Task {
        id: 1,
        name: "Test Task".to_string(),
        completed: false,
    });

    db.save_to_file().expect("Failed to save database");

    match Database::load_from_file() {
        Ok(loaded_db) => println!("Loaded Database: {:?}", loaded_db),
        Err(err) => eprintln!("Error loading database: {}", err),
    }
}
