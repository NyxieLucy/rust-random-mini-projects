use chrono::{DateTime, Utc};
use json::JsonValue;
use serde_json::{Deserializer, Serializer, json};
#[derive(Debug)]
pub struct Task{
    pub title: String,
    pub date: String,
    pub checked : bool,
}
pub fn to_json() -> JsonValue {
    let mut obj = json::JsonValue
}
