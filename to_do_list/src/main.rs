use std::env;
use std::string::String;


use chrono::{Utc};

use crate::structs::Task;
mod structs;
fn main() {
   let mut list:Vec<Task> = Vec::new(); 
     let new_task_title:String =  env::args().skip(1).collect::<Vec<String>>().join(" ");
    let new_date = Utc::now().to_string();
    let new_task = Task{
        title : new_task_title,
        date : new_date, 
        checked : false,
    };
    list.push(new_task);
    for (i, task) in list.iter().enumerate(){
        println!("item #{}: {} [{}]", i + 1, task.title, task.date);
    }
}
