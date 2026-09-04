use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
};

use thiserror::Error;

use super::order::{OrderLocation, RestingOrder};
use super::{
    BookSnapshot, CancelReport, ExecutionReport, NewOrder, OrderId, OrderStatus, Price, PriceLevel,
    Quantity, Side, TimeInForce, Trade, TradeId,
};

pub trait MatchingEngine: Send {
    fn submit(&mut self, order: NewOrder) -> Result<ExecutionReport, BookError>;
    fn cancel(&mut self, order_id: OrderId) -> Result<CancelReport, BookError>;
    fn snapshot(&self, depth: usize) -> BookSnapshot;
}

#[derive(Debug)]
pub struct OrderBook {
    symbol: String,
    bids: BTreeMap<Reverse<Price>, VecDeque<RestingOrder>>,
    asks: BTreeMap<Price, VecDeque<RestingOrder>>,
    active_orders: HashMap<OrderId, OrderLocation>,
    seen_order_ids: HashSet<OrderId>,
    next_trade_id: u64,
    sequence: u64,
}

impl OrderBook {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            active_orders: HashMap::new(),
            seen_order_ids: HashSet::new(),
            next_trade_id: 1,
            sequence: 0,
        }
    }

    fn would_cross(&self, order: &NewOrder) -> bool {
        match order.side {
            Side::Buy => self
                .asks
                .first_key_value()
                .is_some_and(|(best_ask, _)| *best_ask <= order.price),
            Side::Sell => self
                .bids
                .first_key_value()
                .is_some_and(|(Reverse(best_bid), _)| *best_bid >= order.price),
        }
    }

    fn available_liquidity(&self, order: &NewOrder) -> u64 {
        match order.side {
            Side::Buy => self
                .asks
                .range(..=order.price)
                .flat_map(|(_, level)| level)
                .fold(0_u64, |total, resting| {
                    total.saturating_add(resting.remaining_quantity)
                }),
            Side::Sell => self
                .bids
                .iter()
                .take_while(|(Reverse(price), _)| *price >= order.price)
                .flat_map(|(_, level)| level)
                .fold(0_u64, |total, resting| {
                    total.saturating_add(resting.remaining_quantity)
                }),
        }
    }

    fn match_buy(
        &mut self,
        taker_order_id: OrderId,
        limit_price: Price,
        remaining: &mut u64,
        trades: &mut Vec<Trade>,
    ) {
        while *remaining > 0 {
            let Some(best_price) = self.asks.first_key_value().map(|(price, _)| *price) else {
                break;
            };
            if best_price > limit_price {
                break;
            }

            let (maker_order_id, fill_quantity, maker_filled) = {
                let maker = self
                    .asks
                    .get_mut(&best_price)
                    .and_then(VecDeque::front_mut)
                    .expect("best ask level must contain an order");
                let fill_quantity = (*remaining).min(maker.remaining_quantity);
                maker.remaining_quantity -= fill_quantity;
                (maker.order_id, fill_quantity, maker.remaining_quantity == 0)
            };

            *remaining -= fill_quantity;
            if maker_filled {
                let level = self
                    .asks
                    .get_mut(&best_price)
                    .expect("best ask level must exist");
                level.pop_front();
                self.active_orders.remove(&maker_order_id);
                if level.is_empty() {
                    self.asks.remove(&best_price);
                }
            }

            trades.push(self.create_trade(
                maker_order_id,
                taker_order_id,
                Side::Buy,
                best_price,
                fill_quantity,
            ));
        }
    }

    fn match_sell(
        &mut self,
        taker_order_id: OrderId,
        limit_price: Price,
        remaining: &mut u64,
        trades: &mut Vec<Trade>,
    ) {
        while *remaining > 0 {
            let Some(best_price) = self
                .bids
                .first_key_value()
                .map(|(Reverse(price), _)| *price)
            else {
                break;
            };
            if best_price < limit_price {
                break;
            }

            let key = Reverse(best_price);
            let (maker_order_id, fill_quantity, maker_filled) = {
                let maker = self
                    .bids
                    .get_mut(&key)
                    .and_then(VecDeque::front_mut)
                    .expect("best bid level must contain an order");
                let fill_quantity = (*remaining).min(maker.remaining_quantity);
                maker.remaining_quantity -= fill_quantity;
                (maker.order_id, fill_quantity, maker.remaining_quantity == 0)
            };

            *remaining -= fill_quantity;
            if maker_filled {
                let level = self.bids.get_mut(&key).expect("best bid level must exist");
                level.pop_front();
                self.active_orders.remove(&maker_order_id);
                if level.is_empty() {
                    self.bids.remove(&key);
                }
            }

            trades.push(self.create_trade(
                maker_order_id,
                taker_order_id,
                Side::Sell,
                best_price,
                fill_quantity,
            ));
        }
    }

    fn create_trade(
        &mut self,
        maker_order_id: OrderId,
        taker_order_id: OrderId,
        aggressor_side: Side,
        price: Price,
        quantity: u64,
    ) -> Trade {
        let trade = Trade {
            trade_id: TradeId::new(self.next_trade_id),
            maker_order_id,
            taker_order_id,
            aggressor_side,
            price,
            quantity: Quantity::new(quantity).expect("a trade quantity is always positive"),
        };
        self.next_trade_id += 1;
        trade
    }

    fn rest(&mut self, order: &NewOrder, remaining_quantity: u64) {
        let resting = RestingOrder {
            order_id: order.order_id,
            remaining_quantity,
        };
        self.active_orders.insert(
            order.order_id,
            OrderLocation {
                side: order.side,
                price: order.price,
            },
        );

        match order.side {
            Side::Buy => self
                .bids
                .entry(Reverse(order.price))
                .or_default()
                .push_back(resting),
            Side::Sell => self.asks.entry(order.price).or_default().push_back(resting),
        }
    }

    fn levels<'a>(
        levels: impl Iterator<Item = (&'a Price, &'a VecDeque<RestingOrder>)>,
        depth: usize,
    ) -> Vec<PriceLevel> {
        levels
            .take(depth)
            .map(|(price, orders)| PriceLevel {
                price: *price,
                total_quantity: orders.iter().fold(0_u64, |total, order| {
                    total.saturating_add(order.remaining_quantity)
                }),
                order_count: orders.len(),
            })
            .collect()
    }
}

impl MatchingEngine for OrderBook {
    fn submit(&mut self, order: NewOrder) -> Result<ExecutionReport, BookError> {
        if self.seen_order_ids.contains(&order.order_id) {
            return Err(BookError::DuplicateOrder(order.order_id));
        }
        if order.post_only && self.would_cross(&order) {
            return Err(BookError::PostOnlyWouldCross(order.order_id));
        }

        self.seen_order_ids.insert(order.order_id);
        self.sequence += 1;

        let original_quantity = order.quantity;
        if order.time_in_force == TimeInForce::Fok
            && self.available_liquidity(&order) < original_quantity.get()
        {
            return Ok(ExecutionReport {
                order_id: order.order_id,
                status: OrderStatus::Cancelled,
                original_quantity,
                filled_quantity: 0,
                remaining_quantity: original_quantity.get(),
                trades: Vec::new(),
            });
        }

        let mut remaining_quantity = original_quantity.get();
        let mut trades = Vec::new();
        match order.side {
            Side::Buy => self.match_buy(
                order.order_id,
                order.price,
                &mut remaining_quantity,
                &mut trades,
            ),
            Side::Sell => self.match_sell(
                order.order_id,
                order.price,
                &mut remaining_quantity,
                &mut trades,
            ),
        }

        let filled_quantity = original_quantity.get() - remaining_quantity;
        let should_rest = remaining_quantity > 0 && order.time_in_force == TimeInForce::Gtc;
        if should_rest {
            self.rest(&order, remaining_quantity);
        }

        let status = match (filled_quantity, remaining_quantity, should_rest) {
            (_, 0, _) => OrderStatus::Filled,
            (0, _, true) => OrderStatus::Resting,
            (_, _, true) => OrderStatus::PartiallyFilledResting,
            (0, _, false) => OrderStatus::Cancelled,
            (_, _, false) => OrderStatus::PartiallyFilledCancelled,
        };

        Ok(ExecutionReport {
            order_id: order.order_id,
            status,
            original_quantity,
            filled_quantity,
            remaining_quantity,
            trades,
        })
    }

    fn cancel(&mut self, order_id: OrderId) -> Result<CancelReport, BookError> {
        let location = *self
            .active_orders
            .get(&order_id)
            .ok_or(BookError::OrderNotFound(order_id))?;

        let cancelled_quantity = match location.side {
            Side::Buy => {
                let key = Reverse(location.price);
                let level = self
                    .bids
                    .get_mut(&key)
                    .ok_or(BookError::InvariantViolation)?;
                let position = level
                    .iter()
                    .position(|order| order.order_id == order_id)
                    .ok_or(BookError::InvariantViolation)?;
                let removed = level
                    .remove(position)
                    .ok_or(BookError::InvariantViolation)?;
                if level.is_empty() {
                    self.bids.remove(&key);
                }
                removed.remaining_quantity
            }
            Side::Sell => {
                let level = self
                    .asks
                    .get_mut(&location.price)
                    .ok_or(BookError::InvariantViolation)?;
                let position = level
                    .iter()
                    .position(|order| order.order_id == order_id)
                    .ok_or(BookError::InvariantViolation)?;
                let removed = level
                    .remove(position)
                    .ok_or(BookError::InvariantViolation)?;
                if level.is_empty() {
                    self.asks.remove(&location.price);
                }
                removed.remaining_quantity
            }
        };

        self.active_orders.remove(&order_id);
        self.sequence += 1;
        Ok(CancelReport {
            order_id,
            cancelled_quantity,
        })
    }

    fn snapshot(&self, depth: usize) -> BookSnapshot {
        let bids = Self::levels(
            self.bids
                .iter()
                .map(|(Reverse(price), orders)| (price, orders)),
            depth,
        );
        let asks = Self::levels(self.asks.iter(), depth);

        BookSnapshot {
            symbol: self.symbol.clone(),
            sequence: self.sequence,
            active_order_count: self.active_orders.len(),
            trade_count: self.next_trade_id - 1,
            bids,
            asks,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BookError {
    #[error("order {0} has already been used")]
    DuplicateOrder(OrderId),
    #[error("active order {0} was not found")]
    OrderNotFound(OrderId),
    #[error("post-only order {0} would immediately trade")]
    PostOnlyWouldCross(OrderId),
    #[error("order-book indexes are inconsistent")]
    InvariantViolation,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order(id: u64, side: Side, price: u64, quantity: u64) -> NewOrder {
        NewOrder::new(id, side, price, quantity, TimeInForce::Gtc, false).unwrap()
    }

    fn assert_invariants(book: &OrderBook) {
        let mut queue_order_ids = HashSet::new();

        for (Reverse(price), level) in &book.bids {
            assert!(!level.is_empty(), "empty bid level at {}", price.get());
            for resting in level {
                assert!(resting.remaining_quantity > 0);
                assert!(queue_order_ids.insert(resting.order_id));
                assert_eq!(
                    book.active_orders.get(&resting.order_id),
                    Some(&OrderLocation {
                        side: Side::Buy,
                        price: *price,
                    })
                );
            }
        }

        for (price, level) in &book.asks {
            assert!(!level.is_empty(), "empty ask level at {}", price.get());
            for resting in level {
                assert!(resting.remaining_quantity > 0);
                assert!(queue_order_ids.insert(resting.order_id));
                assert_eq!(
                    book.active_orders.get(&resting.order_id),
                    Some(&OrderLocation {
                        side: Side::Sell,
                        price: *price,
                    })
                );
            }
        }

        assert_eq!(queue_order_ids.len(), book.active_orders.len());
        if let (Some((Reverse(best_bid), _)), Some((best_ask, _))) =
            (book.bids.first_key_value(), book.asks.first_key_value())
        {
            assert!(best_bid < best_ask, "resting book must never be crossed");
        }
    }

    #[test]
    fn matches_best_price_before_worse_price() {
        let mut book = OrderBook::new("TEST");
        book.submit(order(1, Side::Sell, 102, 2)).unwrap();
        book.submit(order(2, Side::Sell, 101, 2)).unwrap();

        let report = book.submit(order(3, Side::Buy, 102, 3)).unwrap();

        assert_eq!(report.status, OrderStatus::Filled);
        assert_eq!(report.trades.len(), 2);
        assert_eq!(report.trades[0].maker_order_id, OrderId::new(2).unwrap());
        assert_eq!(report.trades[0].price, Price::new(101).unwrap());
        assert_eq!(report.trades[1].maker_order_id, OrderId::new(1).unwrap());
    }

    #[test]
    fn matches_first_order_at_the_same_price_first() {
        let mut book = OrderBook::new("TEST");
        book.submit(order(1, Side::Sell, 100, 5)).unwrap();
        book.submit(order(2, Side::Sell, 100, 5)).unwrap();

        let report = book.submit(order(3, Side::Buy, 100, 7)).unwrap();

        assert_eq!(report.trades[0].maker_order_id, OrderId::new(1).unwrap());
        assert_eq!(report.trades[0].quantity, Quantity::new(5).unwrap());
        assert_eq!(report.trades[1].maker_order_id, OrderId::new(2).unwrap());
        assert_eq!(report.trades[1].quantity, Quantity::new(2).unwrap());
    }

    #[test]
    fn ioc_cancels_unfilled_remainder() {
        let mut book = OrderBook::new("TEST");
        book.submit(order(1, Side::Sell, 100, 2)).unwrap();
        let ioc = NewOrder::new(2, Side::Buy, 100, 5, TimeInForce::Ioc, false).unwrap();

        let report = book.submit(ioc).unwrap();

        assert_eq!(report.status, OrderStatus::PartiallyFilledCancelled);
        assert_eq!(report.filled_quantity, 2);
        assert_eq!(report.remaining_quantity, 3);
        assert_eq!(book.snapshot(10).active_order_count, 0);
    }

    #[test]
    fn fok_does_not_partially_fill() {
        let mut book = OrderBook::new("TEST");
        book.submit(order(1, Side::Sell, 100, 2)).unwrap();
        let fok = NewOrder::new(2, Side::Buy, 100, 5, TimeInForce::Fok, false).unwrap();

        let report = book.submit(fok).unwrap();

        assert_eq!(report.status, OrderStatus::Cancelled);
        assert!(report.trades.is_empty());
        assert_eq!(book.snapshot(10).asks[0].total_quantity, 2);
    }

    #[test]
    fn post_only_rejects_crossing_order() {
        let mut book = OrderBook::new("TEST");
        book.submit(order(1, Side::Sell, 100, 2)).unwrap();
        let post_only = NewOrder::new(2, Side::Buy, 100, 1, TimeInForce::Gtc, true).unwrap();

        assert_eq!(
            book.submit(post_only),
            Err(BookError::PostOnlyWouldCross(OrderId::new(2).unwrap()))
        );
    }

    #[test]
    fn cancellation_removes_only_requested_order() {
        let mut book = OrderBook::new("TEST");
        book.submit(order(1, Side::Buy, 99, 2)).unwrap();
        book.submit(order(2, Side::Buy, 99, 3)).unwrap();

        let cancelled = book.cancel(OrderId::new(1).unwrap()).unwrap();
        let snapshot = book.snapshot(10);

        assert_eq!(cancelled.cancelled_quantity, 2);
        assert_eq!(snapshot.bids[0].total_quantity, 3);
        assert_eq!(snapshot.bids[0].order_count, 1);
    }

    #[test]
    fn used_order_id_cannot_be_reused_after_fill() {
        let mut book = OrderBook::new("TEST");
        book.submit(order(1, Side::Sell, 100, 1)).unwrap();
        book.submit(order(2, Side::Buy, 100, 1)).unwrap();

        assert_eq!(
            book.submit(order(1, Side::Sell, 101, 1)),
            Err(BookError::DuplicateOrder(OrderId::new(1).unwrap()))
        );
    }

    #[test]
    fn snapshot_is_best_price_first_and_depth_limited() {
        let mut book = OrderBook::new("TEST");
        book.submit(order(1, Side::Buy, 98, 2)).unwrap();
        book.submit(order(2, Side::Buy, 100, 3)).unwrap();
        book.submit(order(3, Side::Sell, 103, 4)).unwrap();
        book.submit(order(4, Side::Sell, 102, 5)).unwrap();

        let snapshot = book.snapshot(1);

        assert_eq!(snapshot.bids.len(), 1);
        assert_eq!(snapshot.bids[0].price, Price::new(100).unwrap());
        assert_eq!(snapshot.asks[0].price, Price::new(102).unwrap());
    }

    #[test]
    fn non_crossing_gtc_order_rests() {
        let mut book = OrderBook::new("TEST");
        let report = book.submit(order(1, Side::Buy, 100, 5)).unwrap();

        assert_eq!(report.status, OrderStatus::Resting);
        assert_eq!(report.remaining_quantity, 5);
        assert_eq!(book.snapshot(10).active_order_count, 1);
    }

    #[test]
    fn invariants_hold_during_deterministic_stress_scenario() {
        let mut book = OrderBook::new("STRESS");
        let mut pseudo_random = 0x5eed_u64;

        for id in 1..=1_000 {
            pseudo_random = pseudo_random
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            let side = if pseudo_random & 1 == 0 {
                Side::Buy
            } else {
                Side::Sell
            };
            let price = 95 + (pseudo_random >> 8) % 11;
            let quantity = 1 + (pseudo_random >> 16) % 20;
            let time_in_force = match (pseudo_random >> 24) % 3 {
                0 => TimeInForce::Gtc,
                1 => TimeInForce::Ioc,
                _ => TimeInForce::Fok,
            };
            let incoming = NewOrder::new(id, side, price, quantity, time_in_force, false).unwrap();
            let report = book.submit(incoming).unwrap();
            assert_eq!(
                report.original_quantity.get(),
                report.filled_quantity + report.remaining_quantity
            );

            if id % 13 == 0 {
                if let Some(active_id) = book.active_orders.keys().next().copied() {
                    book.cancel(active_id).unwrap();
                }
            }
            assert_invariants(&book);
        }
    }
}
