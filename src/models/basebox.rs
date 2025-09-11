use crate::models::item::Item;
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct BoxConstraints {
    pub max_width: u32,
    pub max_height: u32,
    pub max_depth: u32,
    pub max_weight: u32,
    pub max_price: u32,
}

#[derive(Debug, Serialize)]
pub struct BoxInstance {
    pub items: Vec<Item>,
    pub constraints: BoxConstraints,
}

impl BoxInstance {
    pub fn new(constraints: BoxConstraints) -> Self {
        BoxInstance {
            items: Vec::new(),
            constraints,
        }
    }

    pub fn can_add(&self, item: &Item) -> bool {
        let total_width = self.items.iter().map(|i| i.width).max().unwrap_or(0).max(item.width);
        let total_height: u32 = self.items.iter().map(|i| i.height).sum::<u32>() + item.height;
        let total_depth = self.items.iter().map(|i| i.depth).max().unwrap_or(0).max(item.depth);
        let total_weight: u32 = self.items.iter().map(|i| i.weight).sum::<u32>() + item.weight;
        let total_price: u32 = self.items.iter().map(|i| i.price).sum::<u32>() + item.price;

        total_width <= self.constraints.max_width &&
        total_height <= self.constraints.max_height &&
        total_depth <= self.constraints.max_depth &&
        total_weight <= self.constraints.max_weight &&
        total_price <= self.constraints.max_price
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }
}