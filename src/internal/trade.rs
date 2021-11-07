use super::order::Order;
use serde::Serialize;
use std::fs::File;

#[derive(Serialize)]
pub struct Trade(Order, Order);

impl Trade {
    pub fn new(order1: Order, order2: Order) -> Trade {
        Trade(order1, order2)
    }
}

pub struct Trades(Vec<Trade>);

impl Default for Trades {
    fn default() -> Self {
        Trades(Vec::default())
    }
}

impl Trades {
    pub fn add(&mut self, order1: Order, order2: Order) {
        self.0.push(Trade::new(order1, order2));
    }

    pub fn flush(&self) {
        serde_json::to_writer(
            File::create("trades.json").expect("Failed to open file trade"),
            &self.0,
        )
        .expect("Failed to flush trade");
    }
}
