use chrono::{DateTime, Utc};
#[derive(Debug)]
pub struct Task{
    pub task: String,
    pub date: DateTime<Utc>,
    pub checked : bool,
}
