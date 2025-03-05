mod common;
mod operations;
use crate::common::*;
use crate::operations::*; // Import the common module

// Create a new task
async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().expect("Failed to lock database");
    db.insert_task(task.into_inner());
    HttpResponse::Created().json("Task created successfully")
}
async fn add_user(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db = app_state.db.lock().expect("Failed to lock db");
    db.insert_user(user.into_inner());
    HttpResponse::Ok().json("User added")
}

// Get all tasks
async fn get_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().expect("Failed to lock database");
    let tasks = db.get_all_tasks();
    HttpResponse::Ok().json(tasks)
}
async fn delete_task(app_state: web::Data<AppState>, body: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().expect("Failed to lock database");

    if db.delete_task(&body).is_some() {
        HttpResponse::Ok().json("Deleted successfully")
    } else {
        HttpResponse::NotFound().json("Task not found")
    }
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
            .route("/insert_user", web::post().to(add_user))
            .route("/delete/{id}", web::delete().to(delete_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
