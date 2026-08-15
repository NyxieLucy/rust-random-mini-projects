use std::env;
use std::string::String;
use std::fs::File;
use std::io::{BufWriter, Write};
use chrono::Utc;
use crate::structs::CheckState::{self, Checked, Unchecked};
use crate::structs::{Command, Task};
mod structs;

fn main() -> std::io::Result<()> {
    let mut list: Vec<Task> = std::fs::read_to_string("storage.json")
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(Vec::new);

    let mut argument = env::args().skip(1);
    let command_word = argument.next().unwrap_or_default();
    let rest = argument.collect::<Vec<String>>().join(" ");

    let command = match command_word.as_str() {
        "add" => Command::Add(rest),
        "delete" => Command::Delete(rest),
        "check" => Command::Check(rest),
        _ => {eprintln!("no command added :("); return Ok(()); },
    };

    match command {
        Command::Add(title) => {
            let new_task = Task {
                title,
                date: Utc::now().to_string(),
                state: Unchecked,
            };
            list.push(new_task);
        }
        Command::Delete(target) => {
            list.retain(|task| task.title != target);
        }
        Command::Check(target) => {
            if let Some(task) = list.iter_mut().find(|task| task.title == target) {
                task.state = Checked;
            } else {
                eprintln!("no task found matching \"{}\"", target);
            }
        }
    }

    for (i, task) in list.iter().enumerate() {
        println!("item #{}: {} [{:?}]", i + 1, task.title, task.state);
    }

    let storage = File::create("storage.json").unwrap();
    let mut writer = BufWriter::new(storage);
    serde_json::to_writer(&mut writer, &list)?;
    writer.flush()?;
    Ok(())
}
