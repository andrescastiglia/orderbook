use order::internal::{console::Console, order::Order, order_book::OrderBook, trade::Trades};
use log::{error, info, warn};

static LOGGER: Console = Console;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .unwrap();

    let mut order_book = OrderBook::default();
    let mut trades = Trades::default();

    let orders = match Order::from_file("orders.json") {
        Ok(orders) => orders,
        Err(e) => panic!("Couldn't read order.json - {}", e),
    };

    for new_order in orders {
        if new_order.is_new() {
            match order_book.cancel(&new_order) {
                Some(cancel) => info!("{} (Cancel)", cancel),
                None => {
                    if let Some(found) = order_book.take(&new_order) {
                        info!("{} (Commit)", found);
                        trades.add(new_order, found);
                    } else {
                        info!("{} (Add)", new_order);
                        order_book.add(new_order);
                    }
                }
            }
        } else if new_order.is_delete() {
            if let Some(cancel) = order_book.cancel(&new_order) {
                info!("{} (Cancel)", cancel);
            } else {
                info!("{} (add)", new_order);
                order_book.add(new_order);
            }
        } else {
            warn!("Operation type is invalid - {}", new_order);
        }
    }

    if let Err(e) = order_book.flush() {
        error!("Couldn't flush order book - {}", e);
    }

    if let Err(e) = trades.flush() {
        error!("Couldn't flush trades - {}", e);
    }
}
