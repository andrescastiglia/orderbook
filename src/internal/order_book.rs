use super::order::Order;
use serde::Serialize;
use std::fs::File;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Failed to write - {0}")]
    FileError(#[from] std::io::Error),

    #[error("Failed to deserialize - {0}")]
    DeserializeError(#[from] serde_json::Error),
}

#[derive(Serialize)]
pub struct OrderBook(Vec<Order>);

impl Default for OrderBook {
    fn default() -> Self {
        OrderBook(Vec::default())
    }
}

impl OrderBook {
    pub fn add(&mut self, order: Order) {
        let index = self.0.binary_search(&order).unwrap_or_else(|index| index);
        self.0.insert(index, order);
    }

    pub fn take(&mut self, order: &Order) -> Option<Order> {
        self.0
            .iter()
            .position(|item| item.alcoyana(order))
            .map(|index| self.0.remove(index))
    }

    pub fn cancel(&mut self, order: &Order) -> Option<Order> {
        self.0
            .iter()
            .position(|item| item.eq(order) && !item.same_operation(order))
            .map(|index| self.0.remove(index))
    }

    pub fn flush(&self) -> Result<(), FileError> {
        Ok(serde_json::to_writer(
            File::create("orderbook.json")?,
            &self.0,
        )?)
    }
}
