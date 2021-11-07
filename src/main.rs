mod internal;

use internal::{order::Order, order_book::OrderBook, trade::Trades};

fn main() {
    let mut order_book = OrderBook::default();
    let mut trades = Trades::default();

    let orders = Order::from_file("orders.json");

    orders
        .into_iter()
        .for_each(|new_order| match order_book.take(&new_order) {
            Some(found) => trades.add(new_order, found),
            None => order_book.add(new_order),
        });

    order_book.flush();
    trades.flush();
}
