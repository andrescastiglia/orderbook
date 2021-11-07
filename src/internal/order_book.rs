use std::fs::File;

use serde::Serialize;

use super::order::Order;

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

    pub fn flush(&self) {
        serde_json::to_writer(
            File::create("orderbook.json").expect("Failed to open file order_book"),
            &self.0,
        )
        .expect("Failed to flush order_book");
    }
}
