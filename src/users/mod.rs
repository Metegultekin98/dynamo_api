use dynomite::Item;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

mod routes;

pub fn table_name() -> String {
    String::from("users")
}

#[derive(Debug, Serialize, Deserialize, Item, Clone)]
pub struct User {
    #[dynomite(partition_key)]
    pub id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub age: u16,
    pub weight: u16,
    pub description: String,
    is_teacher: bool
}

impl Default for User {
    fn default() -> Self {
        User { 
            id: Some(Uuid::new_v4()),
            first_name: "".to_string(),
            last_name: "".to_string(), 
            age: u16::default(), 
            weight: u16::default(),
            description: "".to_string(), 
            is_teacher: bool::default()
        }
    }
}