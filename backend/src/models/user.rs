use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>, // Optional ID
    pub name: String,
    pub email: String,
}