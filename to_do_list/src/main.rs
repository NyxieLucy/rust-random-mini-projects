use std::env;
use std::string::String;


use chrono::{DateTime};

use crate::structs::Task;
mod structs;
fn main() {
    
    let mut list : Vec<String> = Vec::new();
     let new_task_title:String =  env::args().skip(1).collect();
    let new_Task = Task{
        title : new_task_title,
        checked : false,

    };
    for i in 1..list.len(){
        println!("the item number {i} is {:?}\n ", list[i]);
    }
}
