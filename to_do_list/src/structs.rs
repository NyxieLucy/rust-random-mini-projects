use json::JsonValue;
use std::fs::File;
use std::io::Write;
use serde_json::{Deserializer, Serializer};
#[derive(Debug)]
pub struct Task{
    pub title: String,
    pub date: String,
    pub checked : bool,
}
// adding an impl to add functionalitites to the type we're tring to make 
impl Task {
    // i'm lwk tryna make the added data into json value 
        pub fn to_json(&mut self) -> JsonValue {
        let mut obj = JsonValue::new_object();
        obj["title"] = self.name.clone().into();
        obj["date"] = self.description.clone().into();
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
}

