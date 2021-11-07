use super::{operation::Operation, side::Side};
use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};
use std::{fmt::Display, fs::File, io::Read, str::FromStr};

fn from_str<'a, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'a>,
{
    let text = String::deserialize(deserializer)?;
    T::from_str(&text).map_err(de::Error::custom)
}

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

    pub fn from_file(file_name: &str) -> Vec<Order> {
        let mut buffer = Vec::new();

        File::open(file_name)
            .expect("Failed to open file")
            .read_to_end(&mut buffer)
            .expect("Failed to read file");

        serde_json::from_str(&String::from_utf8(buffer).expect("Failed to parse file"))
            .expect("Failed to parse file")
    }

    pub fn alcoyana(&self, order: &Order) -> bool {
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
    fn capri_capri() {
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

        assert!(order1.alcoyana(&order2));
    }

    #[test]
    fn alcoyana_capri() {
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

        assert!(!order1.alcoyana(&order2));
    }
}
