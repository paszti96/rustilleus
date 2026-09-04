use std::time::Instant;

use rustilleus::domain::{MatchingEngine, NewOrder, OrderBook, Side, TimeInForce};

const ORDER_COUNT: u64 = 200_000;
const MATCH_PRICE: u64 = 10_000;

fn main() {
    let mut book = OrderBook::new("BENCH-USD");
    let started = Instant::now();

    for id in 1..=(ORDER_COUNT / 2) {
        let sell = NewOrder::new(id, Side::Sell, MATCH_PRICE, 1, TimeInForce::Gtc, false)
            .expect("benchmark order is valid");
        book.submit(sell).expect("unique order must be accepted");
    }

    for id in (ORDER_COUNT / 2 + 1)..=ORDER_COUNT {
        let buy = NewOrder::new(id, Side::Buy, MATCH_PRICE, 1, TimeInForce::Gtc, false)
            .expect("benchmark order is valid");
        book.submit(buy).expect("unique order must be accepted");
    }

    let elapsed = started.elapsed();
    let orders_per_second = ORDER_COUNT as f64 / elapsed.as_secs_f64();
    let snapshot = book.snapshot(1);

    println!("processed: {ORDER_COUNT} orders");
    println!("trades: {}", snapshot.trade_count);
    println!("elapsed: {elapsed:.3?}");
    println!("throughput: {orders_per_second:.0} orders/second");
    assert_eq!(snapshot.active_order_count, 0);
    assert_eq!(snapshot.trade_count, ORDER_COUNT / 2);
}
