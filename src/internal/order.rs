use super::{operation::Operation, side::Side};
use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};
use std::{fmt::Display, fs::File, io::Read, str::FromStr, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Failed to read - {0}")]
    FileError(#[from] std::io::Error),

    #[error("Failed to convert bytes - {0}")]
    ConvertBytesError(#[from] FromUtf8Error),

    #[error("Failed to deserialize - {0}")]
    Deserialize(#[from] serde_json::error::Error),
}

fn from_str<'a, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'a>,
{
    let text = String::deserialize(deserializer)?;
    T::from_str(&text).map_err(de::Error::custom)
}

/// Struct that represents a single order
#[derive(Serialize, Deserialize)]
pub struct Order {
    #[serde(deserialize_with = "from_str")]
    type_op: Operation,
    #[serde(deserialize_with = "from_str")]
    account_id: u32,
    #[serde(deserialize_with = "from_str")]
    amount: f64,
    #[serde(deserialize_with = "from_str")]
    order_id: u32,
    pair: String,
    #[serde(deserialize_with = "from_str")]
    limit_price: f64,
    #[serde(deserialize_with = "from_str")]
    side: Side,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} #{}", self.type_op, self.side, self.order_id)
    }
}

impl Eq for Order {}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.order_id.eq(&other.order_id)
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.amount.partial_cmp(&other.amount).unwrap()
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

impl Order {
    fn is_buy(&self) -> bool {
        self.side.eq(&Side::Buy)
    }
    fn is_sell(&self) -> bool {
        self.side.eq(&Side::Sell)
    }
    #[must_use] 
    pub fn is_new(&self) -> bool {
        self.type_op.eq(&Operation::Create)
    }
    #[must_use] 
    pub fn is_delete(&self) -> bool {
        self.type_op.eq(&Operation::Delete)
    }
    #[must_use] 
    pub fn same_operation(&self, order: &Order) -> bool {
        self.type_op.eq(&order.type_op)
    }

    /// Load orders from a file
    /// # Arguments
    /// * `file_name` - The file name to load
    /// # Returns
    /// * `Vec<Order>` - The orders loaded from the file
    /// # Errors
    /// * `FileError` - If the file cannot be read
    pub fn from_file(file_name: &str) -> Result<Vec<Order>, FileError> {
        let mut buffer = Vec::default();
        File::open(file_name)?.read_to_end(&mut buffer)?;
        Ok(serde_json::from_str(&String::from_utf8(buffer)?)?)
    }

    /// Match two orders, see also <https://www.youtube.com/watch?v=UMSuvFOExhk>
    /// # Arguments
    /// * `order` - Order to match
    /// # Returns
    /// * `bool` - The matched orders
    #[must_use] 
    pub fn match_with(&self, order: &Order) -> bool {
        if self.is_buy() && order.is_sell() {
            self.amount.ge(&order.amount)
        } else if order.is_buy() && self.is_sell() {
            order.amount.ge(&self.amount)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_ok() {
        let order1 = Order {
            type_op: Operation::Create,
            account_id: 1,
            amount: 1.0,
            order_id: 1,
            pair: "EURUSD".to_string(),
            limit_price: 1.0,
            side: Side::Buy,
        };

        let order2 = Order {
            type_op: Operation::Create,
            account_id: 1,
            amount: 1.0,
            order_id: 1,
            pair: "EURUSD".to_string(),
            limit_price: 1.0,
            side: Side::Sell,
        };

        assert!(order1.match_with(&order2));
    }

    #[test]
    fn not_match() {
        let order1 = Order {
            type_op: Operation::Create,
            account_id: 1,
            amount: 1.0,
            order_id: 1,
            pair: "EURUSD".to_string(),
            limit_price: 1.0,
            side: Side::Buy,
        };

        let order2 = Order {
            type_op: Operation::Create,
            account_id: 1,
            amount: 3.0,
            order_id: 1,
            pair: "EURUSD".to_string(),
            limit_price: 1.0,
            side: Side::Sell,
        };

        assert!(!order1.match_with(&order2));
    }
}
