use std::fs::File;
use std::io::{Read, Write};
use json::JsonValue;

pub struct Task {
    pub name: String,
    pub description: String,
    pub date: String,
    pub completed: bool, // NEW: Track completion status
}

impl Task {
    pub fn new(name: String, description: String, date: String, completed: bool) -> Self {
        Self { name, description, date, completed }
    }

    pub fn to_json(&self) -> JsonValue {
        let mut obj = JsonValue::new_object();
        obj["name"] = self.name.clone().into();
        obj["description"] = self.description.clone().into();
        obj["date"] = self.date.clone().into();
        obj["completed"] = self.completed.into();
        obj
    }
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), std::io::Error> {
    let mut json_array = JsonValue::new_array();
    for task in tasks {
        let _ = json_array.push(task.to_json());
    }
    let mut file = File::create("tasks.json")?;
    file.write_all(json_array.dump().as_bytes())?;
    Ok(())
}

pub fn load_tasks() -> Vec<Task> {
    let mut file = match File::open("tasks.json") {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return Vec::new();
    }

    match json::parse(&contents) {
        Ok(parsed_json) => {
            let mut task_list = Vec::new();
            if parsed_json.is_array() {
                for item in parsed_json.members() {
                    let name = item["name"].as_str().unwrap_or("No Name").to_string();
                    let description = item["description"].as_str().unwrap_or("No Description").to_string();
                    let date = item["date"].as_str().unwrap_or("Unknown Date").to_string();
                    let completed = item["completed"].as_bool().unwrap_or(false);
                    
                    task_list.push(Task::new(name, description, date, completed));
                }
            }
            task_list
        }
        Err(_) => Vec::new(),
    }
}
