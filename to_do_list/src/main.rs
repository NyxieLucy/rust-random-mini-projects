use std::env;
use std::string::String;

use chrono::{Utc};

use crate::structs::Task;
mod structs;
fn main() {
    
    let mut list : Vec<String> = Vec::new();

    let new_task_title:String =  env::args().skip(1).collect();
    let new_date = Utc::now();
    let new_task = Task{
        task: new_task_title.clone(),
        date : new_date,
        checked: false,
    };
    list.push(new_task_title);
    println!("{:?}", new_task);
}
