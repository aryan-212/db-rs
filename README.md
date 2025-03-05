# Task Management API

This is a simple task management API built using Rust and Actix-web. It provides endpoints to create tasks, retrieve tasks, add users, and delete tasks, all while storing data in an in-memory database protected by a mutex.

## Features

* Create a new task
* Retrieve all tasks
* Add a new user
* Delete a task
* CORS support

## Endpoints

### Create a Task

* **URL:** `/task`
* **Method:** `POST`
* **Request Body:** (JSON)
  ```json
  {
    "id": 1,
    "title": "Complete assignment",
    "description": "Finish the Rust project",
    "completed": false
  }
  ```
* **Response:**
  ```json
  "Task created successfully"
  ```
* **Curl Command:**
  ```sh
  curl -X POST http://127.0.0.1:8080/task \
       -H "Content-Type: application/json" \
       -d '{"id":1, "title":"Complete assignment", "description":"Finish the Rust project", "completed":false}'
  ```

### Retrieve All Tasks

* **URL:** `/tasks`
* **Method:** `GET`
* **Response:**
  ```json
  [
    {
      "id": 1,
      "title": "Complete assignment",
      "description": "Finish the Rust project",
      "completed": false
    }
  ]
  ```
* **Curl Command:**
  ```sh
  curl -X GET http://127.0.0.1:8080/tasks
  ```

### Add a User

* **URL:** `/insert_user`
* **Method:** `PUT`
* **Request Body:** (JSON)
  ```json
  {
    "id": 1,
    "name": "John Doe"
  }
  ```
* **Response:**
  ```json
  "User added"
  ```
* **Curl Command:**
  ```sh
  curl -X PUT http://127.0.0.1:8080/insert_user \
       -H "Content-Type: application/json" \
       -d '{"id":1, "username":"John Doe","password":"random@123"}'
  ```

### Delete a Task

* **URL:** `/delete/{id}`
* **Method:** `DELETE`
* **Response (if successful):**
  ```json
  "Deleted successfully"
  ```
* **Response (if not found):**
  ```json
  "Task not found"
  ```
* **Curl Command:**
  ```sh
  curl -X DELETE http://127.0.0.1:8080/delete/1
  ```

## Running the API

1. Clone the repository.
2. Ensure you have Rust installed.
3. Run the following command:
   ```sh
   cargo run
   ```
4. The server will start on `127.0.0.1:8080`.

Now you can use the API to manage tasks and users effectively!
