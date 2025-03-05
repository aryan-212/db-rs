use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
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
    fn insert_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
        self.save_to_file().expect("Failed to save task");
    }

    fn get_task(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn get_all_tasks(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }

    fn delete_task(&mut self, id: &u64) {
        self.tasks.remove(id);
        self.save_to_file().expect("Failed to save after deletion");
    }

    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
        self.save_to_file().expect("Failed to save user");
    }

    fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == username)
    }

    // Save data to file
    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // Load data from file
    fn load_from_file() -> std::io::Result<Self> {
        let mut file = File::open("database.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let db: Database = serde_json::from_str(&contents).unwrap_or(Database::new());
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}

// Create a new task
async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().expect("Failed to lock database");
    db.insert_task(task.into_inner());
    HttpResponse::Created().json("Task created successfully")
}

// Get all tasks
async fn get_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().expect("Failed to lock database");
    let tasks = db.get_all_tasks();
    HttpResponse::Ok().json(tasks)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::load_from_file().unwrap_or_else(|_| Database::new());
    let data = web::Data::new(AppState { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/task", web::post().to(create_task))
            .route("/tasks", web::get().to(get_tasks))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
