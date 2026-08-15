use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub title: String,
    pub date: String,
    pub checked : bool,
}
