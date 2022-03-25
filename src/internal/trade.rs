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

/// Trade is a struct that represents a trade in the market
#[derive(Serialize)]
pub struct Trade(Order, Order);

impl Trade {
    /// Create a new trade
    /// # Arguments
    /// * `order1` - First order
    /// * `seller` - Order opposite to previous
    /// # Returns
    /// A new trade
    pub fn new(order1: Order, order2: Order) -> Trade {
        Trade(order1, order2)
    }
}

#[derive(Default)]
pub struct Trades(Vec<Trade>);

impl Trades {
    /// Add a trade to the list of trades
    /// # Arguments
    /// * `order1` - Order to add, sell or buy
    /// * `order2` - Order to add, oposite of order1
    pub fn add(&mut self, order1: Order, order2: Order) {
        self.0.push(Trade::new(order1, order2));
    }

    /// Write the trades to a file
    /// # Errors
    /// * `FileError` - If the file cannot be written to
    pub fn flush(&self) -> Result<(), FileError> {
        Ok(serde_json::to_writer(
            File::create("trades.json")?,
            &self.0,
        )?)
    }
}
