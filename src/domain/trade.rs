use serde::Serialize;

use super::{OrderId, Price, Quantity, Side};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct TradeId(u64);

impl TradeId {
    pub(crate) const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Trade {
    pub trade_id: TradeId,
    pub maker_order_id: OrderId,
    pub taker_order_id: OrderId,
    pub aggressor_side: Side,
    pub price: Price,
    pub quantity: Quantity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Filled,
    Resting,
    PartiallyFilledResting,
    Cancelled,
    PartiallyFilledCancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExecutionReport {
    pub order_id: OrderId,
    pub status: OrderStatus,
    pub original_quantity: Quantity,
    pub filled_quantity: u64,
    pub remaining_quantity: u64,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CancelReport {
    pub order_id: OrderId,
    pub cancelled_quantity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PriceLevel {
    pub price: Price,
    pub total_quantity: u64,
    pub order_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BookSnapshot {
    pub symbol: String,
    pub sequence: u64,
    pub active_order_count: usize,
    pub trade_count: u64,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}
