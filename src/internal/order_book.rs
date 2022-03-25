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

/// Collection of orders
#[derive(Default, Serialize)]
pub struct OrderBook(Vec<Order>);

impl OrderBook {
    /// Add an order to the order book
    /// # Arguments
    /// * `order` - The order to add
    pub fn add(&mut self, order: Order) {
        let index = self.0.binary_search(&order).unwrap_or_else(|index| index);
        self.0.insert(index, order);
    }

    /// Remove an opposite order from the order book
    /// # Arguments
    /// * `order` - The order to compare against the opposite order
    /// # Returns
    /// * `Option<Order>` - The opposite order if it exists
    pub fn take(&mut self, order: &Order) -> Option<Order> {
        self.0
            .iter()
            .position(|item| item.alcoyana(order))
            .map(|index| self.0.remove(index))
    }

    /// Remove an original order from the order book
    /// # Arguments
    /// * `order` - The order to compare against the original order
    /// # Returns
    /// * `Option<Order>` - The original order if it exists
    pub fn cancel(&mut self, order: &Order) -> Option<Order> {
        self.0
            .iter()
            .position(|item| item.eq(order) && !item.same_operation(order))
            .map(|index| self.0.remove(index))
    }

    /// Write the order book to a file
    /// # Returns
    /// * `Result<(), FileError>` - The result of the write operation
    pub fn flush(&self) -> Result<(), FileError> {
        Ok(serde_json::to_writer(
            File::create("orderbook.json")?,
            &self.0,
        )?)
    }
}
