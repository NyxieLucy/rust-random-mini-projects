use std::env;
use std::string::String;


use chrono::{DateTime, Utc};

use crate::structs::Task;
mod structs;
fn main() {
    
    let mut list : Vec<String> = Vec::new();
     let new_task_title:String =  env::args().skip(1).collect::<Vec<String>>().join(" ");
    let new_date = Utc::now().to_string();
    let new_Task = Task{
        title : new_task_title,
        date : new_date, 
        checked : false,

    };
    for (i, task) in list.iter().enumerate(){
        println!("the item number {} is {:?}\n ",i + 1,  new_Task.title);
    }
}
