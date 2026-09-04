//! Core exchange concepts and matching rules.

mod book;
mod order;
mod trade;

pub use book::{BookError, MatchingEngine, OrderBook};
pub use order::{NewOrder, OrderId, OrderValidationError, Price, Quantity, Side, TimeInForce};
pub use trade::{
    BookSnapshot, CancelReport, ExecutionReport, OrderStatus, PriceLevel, Trade, TradeId,
};
