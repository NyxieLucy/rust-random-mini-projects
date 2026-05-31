pub mod funtions;
pub mod account;

use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};
use crate::account::Account;

enum AppMode {
    Normal,
    InputCin,
    InputName,
    InputDept,
    InputResidency,
    InputSalary,
    InputPromotion,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut accounts: Vec<Account> = funtions::load_accounts();
    let mut current_mode = AppMode::Normal;
    
    let mut input_buffer = String::new();
    let mut temp_cin = String::new();
    let mut temp_name = String::new();
    let mut temp_dept = String::new();
    let mut temp_residency = String::new();
    let mut temp_salary = String::new();

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                let main_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0)])
                    .split(frame.area());

                // --- HEADER ---
                let header_text = match current_mode {
                    AppMode::Normal => "SALARY MGMT | [n] New User | [q] Quit",
                    AppMode::InputCin => "愛 STEP 1: Type CIN and press [Enter]",
                    AppMode::InputName => "愛 STEP 2: Type Name and press [Enter]",
                    AppMode::InputDept => "愛 STEP 3: Type Department and press [Enter]",
                    AppMode::InputResidency => "愛 STEP 4: Type Residency and press [Enter]",
                    AppMode::InputSalary => "愛 STEP 5: Type Salary and press [Enter]",
                    AppMode::InputPromotion => "愛 STEP 6: Next Promotion Date (or press Enter to skip)",
                };
                
                frame.render_widget(
                    Paragraph::new(header_text).cyan().block(Block::bordered().title(" DASHBOARD ")),
                    main_layout[0]
                );

                // --- CONTENT ---
                match current_mode {
                    AppMode::Normal => {
                        let mut list_content = String::new();
                        if accounts.is_empty() {
                            list_content = ".⋆♱ No users found! ₊˚⊹\nPress [n] to add a new employee.".to_string();
                        } else {
                            for (i, acc) in accounts.iter().enumerate() {
                                list_content.push_str(&format!("{}. {} | {} | {} | ${:.2}\n", i+1, acc.cin, acc.name, acc.department, acc.salary));
                            }
                        }
                        frame.render_widget(
                            Paragraph::new(list_content).block(Block::bordered().title(" EMPLOYEES ")),
                            main_layout[1]
                        );
                    }
                    _ => {
                        let prompt = format!("Current Input: {}\n\n[Esc] to Cancel input", input_buffer);
                        frame.render_widget(
                            Paragraph::new(prompt).yellow().block(Block::bordered().title(" DATA ENTRY ")),
                            main_layout[1]
                        );
                    }
                }
            })?;

            // keyboard event handling
            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    match current_mode {
                        AppMode::Normal => match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('n') => {
                                current_mode = AppMode::InputCin;
                                input_buffer.clear();
                            }
                            _ => {}
                        },
                        _ => match key.code {
                            KeyCode::Esc => current_mode = AppMode::Normal,
                            KeyCode::Backspace => { input_buffer.pop(); }
                            KeyCode::Char(c) => { input_buffer.push(c); }
                            KeyCode::Enter => {
                                match current_mode {
                                    AppMode::InputCin => {
                                        temp_cin = input_buffer.clone();
                                        current_mode = AppMode::InputName;
                                    }
                                    AppMode::InputName => {
                                        temp_name = input_buffer.clone();
                                        current_mode = AppMode::InputDept;
                                    }
                                    AppMode::InputDept => {
                                        temp_dept = input_buffer.clone();
                                        current_mode = AppMode::InputResidency;
                                    }
                                    AppMode::InputResidency => {
                                        temp_residency = input_buffer.clone();
                                        current_mode = AppMode::InputSalary;
                                    }
                                    AppMode::InputSalary => {
                                        temp_salary = input_buffer.clone();
                                        current_mode = AppMode::InputPromotion;
                                    }
                                    AppMode::InputPromotion => {
                                        // Parse salary from the previous buffer step if needed, 
                                        // but here we just grab the buffer from the salary step:
                                        let salary = temp_salary.parse::<f64>().unwrap_or(0.0); 
                                        let promo = if input_buffer.is_empty() { None } else { Some(input_buffer.clone()) };
                                        
                                        accounts.push(Account::new(temp_cin.clone(), temp_dept.clone(), temp_residency.clone(), temp_name.clone(), salary, true, false, promo));
                                        let _ = funtions::save_accounts(&accounts);
                                        current_mode = AppMode::Normal;
                                    }
                                    _ => {}
                                }
                                input_buffer.clear();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok::<(), std::io::Error>(())
    })?;

    println!("Exiting Salary Manager...");
    Ok(())
}
