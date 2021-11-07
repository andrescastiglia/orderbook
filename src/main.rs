mod internal;

use internal::{order::Order, order_book::OrderBook, trade::Trades};

fn main() {
    let mut order_book = OrderBook::default();
    let mut trades = Trades::default();

    let orders = Order::from_file("orders.json");

    for new_order in orders {
        if new_order.is_new() {
            if order_book.cancel(&new_order).is_none() {
                match order_book.take(&new_order) {
                    Some(found) => trades.add(new_order, found),
                    None => order_book.add(new_order),
                }
            }
        } else if new_order.is_delete() {
            if order_book.cancel(&new_order).is_none() {
                order_book.add(new_order);
            }
        } else {
            panic!("Operation type is invalid");
        }
    }

    order_book.flush();
    trades.flush();
}
