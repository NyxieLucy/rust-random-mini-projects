use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub cin: String,
    pub department: String,
    pub residency: String,
    pub name: String,
    pub salary: f64,
    pub is_enrolled: bool,
    pub on_vacation: bool,
    pub next_promotion: Option<String>,
}

impl Account {
    pub fn new(
        cin: String,
        department: String,
        residency: String,
        name: String,
        salary: f64,
        is_enrolled: bool,
        on_vacation: bool,
        next_promotion: Option<String>,
    ) -> Self {
        Self {
            cin,
            department,
            residency,
            name,
            salary,
            is_enrolled,
            on_vacation,
            next_promotion,
        }
    }
}
