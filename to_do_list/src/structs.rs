use chrono::{DateTime, Utc};
use serde_json::{Deserializer, Serializer, json};
#[derive(Debug)]
pub struct Task{
    pub title: String,
    pub date: String,
    pub checked : bool,
}
