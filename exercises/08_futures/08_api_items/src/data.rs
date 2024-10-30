use serde::{Deserialize, Serialize};

// Define a simple struct to represent an item
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Item {
    pub id: u32,
    pub name: String,
}
