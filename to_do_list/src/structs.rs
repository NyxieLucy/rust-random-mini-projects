use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub title: String,
    pub date: String,
    pub state: CheckState,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum CheckState {
    Checked ,
    Unchecked ,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Add(String),
    Delete(String),
    Check(String),
}
