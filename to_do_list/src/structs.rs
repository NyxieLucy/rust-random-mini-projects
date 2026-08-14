use json::JsonValue;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub title: String,
    pub date: String,
    pub checked : bool,
}
// adding an impl to add functionalitites to the type we're tring to make 
impl Task {
    // i'm lwk tryna make the added data into json value
    pub fn to_json(&self) -> Value {
        json!({
            "title" :self.title,
            "date" : self.date,
            "checked" : self.checked,
        })
    }
}

