use std::fs;
use std::path::Path;
use crate::account::Account;
const FILE_PATH: &str = "accounts.json";

pub fn save_accounts(accounts: &Vec<Account>) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(accounts)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(FILE_PATH, json)?;
    Ok(())
}

pub fn load_accounts() -> Vec<Account> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}