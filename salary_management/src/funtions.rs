use std::io::{self, Write};

pub fn input_shortcut() {
    let mut input = String::new();
    println!("Please enter a value:");
    io::stdin().read_line(&mut input).expect("failed to enter the value");
    io::stdout().flush().expect("unfortunate");
}
pub fn new_user() {
    println!("Creating a new user...");
    let mut cin = input_shortcut();
    let mut name = input_shortcut();
    let mut department = input_shortcut();
    let mut salary_str = input_shortcut();
    let mut residency = input_shortcut();
    let mut next_promotion = input_shortcut();
    let salary: f64 = salary_str.
    let account = Account::new(
        cin,
        department,
        residency,
        name,
        salary,
        true,
        false,
        Some(next_promotion),
    );
    println!("New user created: {:?}", account);
    
}