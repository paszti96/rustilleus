use rustilleus::domain::{
    MatchingEngine, NewOrder, OrderBook, OrderId, OrderStatus, Price, Side, TimeInForce,
};

fn order(id: u64, side: Side, price: u64, quantity: u64) -> NewOrder {
    NewOrder::new(id, side, price, quantity, TimeInForce::Gtc, false).unwrap()
}

#[test]
fn aggressive_order_sweeps_levels_and_rests_its_remainder() {
    let mut book = OrderBook::new("ETH-USD");
    book.submit(order(1, Side::Sell, 100, 2)).unwrap();
    book.submit(order(2, Side::Sell, 101, 3)).unwrap();
    book.submit(order(3, Side::Sell, 102, 5)).unwrap();

    let report = book.submit(order(4, Side::Buy, 101, 7)).unwrap();
    let snapshot = book.snapshot(10);

    assert_eq!(report.status, OrderStatus::PartiallyFilledResting);
    assert_eq!(report.filled_quantity, 5);
    assert_eq!(report.remaining_quantity, 2);
    assert_eq!(report.trades.len(), 2);
    assert_eq!(report.trades[0].price, Price::new(100).unwrap());
    assert_eq!(report.trades[1].price, Price::new(101).unwrap());
    assert_eq!(snapshot.bids[0].price, Price::new(101).unwrap());
    assert_eq!(snapshot.bids[0].total_quantity, 2);
    assert_eq!(snapshot.asks[0].price, Price::new(102).unwrap());
}

#[test]
fn partial_maker_fill_can_be_cancelled_for_exact_remainder() {
    let mut book = OrderBook::new("ETH-USD");
    book.submit(order(10, Side::Sell, 100, 10)).unwrap();
    book.submit(order(11, Side::Buy, 100, 4)).unwrap();

    let cancelled = book.cancel(OrderId::new(10).unwrap()).unwrap();

    assert_eq!(cancelled.cancelled_quantity, 6);
    assert_eq!(book.snapshot(10).active_order_count, 0);
}

#[test]
fn maker_price_improves_an_aggressive_order() {
    let mut book = OrderBook::new("ETH-USD");
    book.submit(order(20, Side::Sell, 95, 2)).unwrap();

    let report = book.submit(order(21, Side::Buy, 100, 2)).unwrap();

    assert_eq!(report.status, OrderStatus::Filled);
    assert_eq!(report.trades[0].price, Price::new(95).unwrap());
    assert_eq!(report.trades[0].maker_order_id, OrderId::new(20).unwrap());
    assert_eq!(report.trades[0].taker_order_id, OrderId::new(21).unwrap());
}
