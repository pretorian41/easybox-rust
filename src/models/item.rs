use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub weight: u32,
    pub price: u32,
}
