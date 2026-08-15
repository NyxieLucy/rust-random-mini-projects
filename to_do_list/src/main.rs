use std::env;
use std::string::String;
use std::fs::File;
use std::io::{BufWriter, Write};


use chrono::{Utc};

use crate::structs::Task;
mod structs;
fn main() -> std::io::Result<()> {
    let mut list: Vec<Task> = std::fs::read_to_string("storage.json").ok().and_then(|content| serde_json::from_str(&content).ok()).unwrap_or_else(Vec::new);

    let new_task_title: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let new_date = Utc::now().to_string();
    let new_task = Task {
        title: new_task_title,
        date: new_date,
        checked: false,
    };
    list.push(new_task);

    for (i, task) in list.iter().enumerate() {
        println!("item #{}: {} [{}]", i + 1, task.title, task.date);
    }

    let storage = File::create("storage.json").unwrap();
    let mut writer = BufWriter::new(storage);
    serde_json::to_writer(&mut writer, &list)?;
    writer.flush()?;

    Ok(())
}
