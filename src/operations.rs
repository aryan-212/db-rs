use crate::common::*;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: u64,
    username: String,
    password: String,k
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    pub fn insert_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
        self.save_to_file().expect("Failed to save task");
    }

    pub fn get_task(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn get_all_tasks(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }

    pub fn delete_task(&mut self, id: &u64) -> Option<Task> {
        let task = self.tasks.remove(id); // Remove task if it exists
        if task.is_some() {
            self.save_to_file().expect("Failed to save after deletion");
        }
        task 
    }

    pub fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
        self.save_to_file().expect("Failed to save user");
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == username)
    }

    // Save data to file
    pub fn save_to_file(&self) -> std::io::Result<()> {
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
    pub fn load_from_file() -> std::io::Result<Self> {
        let mut file = File::open("database.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let db: Database = serde_json::from_str(&contents).unwrap_or(Database::new());
        Ok(db)
    }
}

pub struct AppState {
    pub db: Mutex<Database>,
}
