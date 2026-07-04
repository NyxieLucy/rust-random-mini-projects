use std::io::Error;
pub mod notes;
use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph, Wrap};
use crate::notes::{Task, load_tasks, save_tasks};
use std::string::String;
use std::time::SystemTime;
use chrono::{DateTime, Utc};

enum AppMode {
    Normal,
    InputName,
    InputDesc, // new state typing w kda
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks: Vec<Task> = load_tasks(); // task vector
    let mut current_mode = AppMode::Normal;
    
    //buffers to step through input fields
    let mut temp_name = String::new();
    let mut temp_desc = String::new();
    let mut input_buffer = String::new();
    
    let mut selected_index: usize = 0;
    let mut is_modifying = false;

    ratatui::run(|terminal| {
        loop {
            if !tasks.is_empty() && selected_index >= tasks.len() {
                selected_index = tasks.len() - 1;
            }

            terminal.draw(|frame| {
                // Main layout cut: Top header row vs Main working grid canvas
                let main_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0)])
                    .split(frame.area());

                // the dashboard header
                let header_text = match current_mode {
                    AppMode::Normal => "welcome, nyxie | [n] New | [c] Check/Uncheck | [d] Delete | [u] Update | [↑/↓] Navigate | [q] Quit",
                    AppMode::InputName => "愛 STEP 1: Type Task NAME and press [Enter] | [Esc] Cancel",
                    AppMode::InputDesc => "愛 STEP 2: Type Task DESCRIPTION and press [Enter] | [Esc] Cancel",
                };
                let title_block = Block::bordered().title(" DASHBOARD ");
                let title = Paragraph::new(header_text).green().block(title_block);
                frame.render_widget(title, main_layout[0]);

                // horizontal splits for list andd detailz
                let workspace = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(main_layout[1]);

                let list_block = Block::bordered().title(" TASKS ");
                let detail_block = Block::bordered().title(" TASK DETAILS INSPECTOR ");

                match current_mode {
                    AppMode::Normal => {
                        // left panel for the task w kda m3aha des checkboxes
                        let list_text = if tasks.is_empty() {
                            ".⋆♱ No tasks found! ₊˚⊹ ᰔ \n Press [n] to create a new task.".to_string()
                        } else {
                            let mut formatted_list = String::new();
                            for (i, item) in tasks.iter().enumerate() {
                                let check = if item.completed { "[✓]" } else { "[ ]" };
                                if i == selected_index {
                                    formatted_list.push_str(&format!("▶ {} {} {}\n", check, i + 1, item.name.clone()));
                                } else {
                                    formatted_list.push_str(&format!("  {} {} {}\n", check, i + 1, item.name));
                                }
                            }
                            formatted_list
                        };
                        let list_widget = Paragraph::new(list_text)
                            .left_aligned()
                            .blue()
                            .block(list_block)
                            .wrap(Wrap { trim: false });
                        frame.render_widget(list_widget, workspace[0]);

                        // RIGHT PANEL: Build the detailed task sheet cards
                        let detail_text = if tasks.is_empty() {
                            "No item selected.".to_string()
                        } else {
                            let current_task = &tasks[selected_index];
                            let status = if current_task.completed { "Completed ꉂ(˵˃ ᗜ ˂˵)" } else { "Pending (｡ᵕ ◞ _◟)" };
                            format!(
                                "Name: {}\n\nCreated: {}\n\nStatus: {}\n\nDescription:\n{}",
                                current_task.name, current_task.date, status, current_task.description
                            )
                        };
                        let detail_widget = Paragraph::new(detail_text)
                            .left_aligned()
                            .yellow()
                            .block(detail_block)
                            .wrap(Wrap { trim: false });
                        frame.render_widget(detail_widget, workspace[1]);
                    }
                    
                    // INPUT SYSTEM DRAWS
                    AppMode::InputName | AppMode::InputDesc => {
                        let prompt_title = match current_mode {
                            AppMode::InputName => {
                                if is_modifying { "Modifying Task -> Name Input Field" } else { "Creating Task -> Name Input Field" }
                            }
                            _ => {
                                if is_modifying { "Modifying Task -> Description Input Field" } else { "Creating Task -> Description Input Field" }
                            }
                        };
                        let input_display = format!("> {}\n\n(Type characters and hit Enter)", input_buffer);
                        let input_widget = Paragraph::new(input_display)
                            .blue()
                            .block(Block::bordered().title(prompt_title))
                            .wrap(Wrap { trim: false });
                        
                        // stretch input layout across full split zone for easier reading
                        frame.render_widget(input_widget, main_layout[1]);
                    }
                }
            })?;

            // Keyboard Tracker Processing Loop
            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    match current_mode {
                        AppMode::Normal => match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('n') => {
                                current_mode = AppMode::InputName;
                                input_buffer.clear();
                            }
                            KeyCode::Up => {
                                if selected_index > 0 { selected_index -= 1; }
                            }
                            KeyCode::Down => {
                                if !tasks.is_empty() && selected_index < tasks.len() - 1 { selected_index += 1; }
                            }
                            // CHECKBOX ACTION: Toggle item complete boolean state flag
                            KeyCode::Char('c') => {
                                if !tasks.is_empty() {
                                    tasks[selected_index].completed = !tasks[selected_index].completed;
                                    let _ = save_tasks(&tasks);
                                }
                            }
                            KeyCode::Char('u') => {
                                if !tasks.is_empty() {
                                    is_modifying = true;
                                    temp_name = tasks[selected_index].name.clone();
                                    temp_desc = tasks[selected_index].description.clone();
                                    input_buffer = tasks[selected_index].name.clone();
                                    current_mode = AppMode::InputName;
                                }
                            }
                            // DELETE ACTION
                            KeyCode::Char('d') => {
                                if !tasks.is_empty() {
                                    tasks.remove(selected_index);
                                    let _ = save_tasks(&tasks);
                                    if selected_index >= tasks.len() {
                                        selected_index = tasks.len().saturating_sub(1);
                                    }
                                }
                            }
                            _ => {}
                        },
                        // input mode 
                        AppMode::InputName => match key.code {
                            KeyCode::Enter => {
                                if !input_buffer.is_empty() {
                                    temp_name = input_buffer.clone();
                                    if is_modifying {
                                        input_buffer = temp_desc.clone();
                                    } else {
                                        input_buffer.clear();
                                    }
                                    current_mode = AppMode::InputDesc;
                                }
                            }
                            KeyCode::Esc => {
                                is_modifying = false;
                                current_mode = AppMode::Normal;
                            }
                            KeyCode::Backspace => { input_buffer.pop(); }
                            KeyCode::Char(c) => { input_buffer.push(c); }
                            _ => {}
                        },
                        AppMode::InputDesc => match key.code {
                            KeyCode::Enter => {
                                if !input_buffer.is_empty() {
                                    temp_desc = input_buffer.clone();
                                    if is_modifying {
                                        tasks[selected_index].name = temp_name.clone();
                                        tasks[selected_index].description = temp_desc.clone();
                                        let _ = save_tasks(&tasks);
                                        is_modifying = false;
                                        current_mode = AppMode::Normal;
                                    } else {
                                        let now = SystemTime::now();
                                        let datetime: DateTime<Utc> = now.into();
                                        let date_stamp = datetime.format("%Y-%m-%d %H:%M:%S %Z").to_string();
                                        let new_task = Task::new(temp_name.clone(), temp_desc.clone(), date_stamp, false);
                                        tasks.push(new_task);
                                        let _ = save_tasks(&tasks);
                                        selected_index = tasks.len() - 1;
                                        current_mode = AppMode::Normal;
                                    }
                                }
                            }
                            KeyCode::Esc => {
                                is_modifying = false;
                                current_mode = AppMode::Normal;
                            }
                            KeyCode::Backspace => { input_buffer.pop(); }
                            KeyCode::Char(c) => { input_buffer.push(c); }
                            _ => {}
                        },
                    }
                }
            }
        }
        Ok::<(), Error>(())
    })?;

    println!("Goodbye! thanks for using *wink wink*!");
    Ok(())
}
