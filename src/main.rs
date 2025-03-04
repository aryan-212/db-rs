use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, http::header, web};
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}
struct User {
    id: u64,
    username: String,
    password: String,
}
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
    // CRUD DATA
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }
    fn get(&mut self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }
    fn get_all(&mut self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }
}
fn main() {
    println!("Hello World");
}
