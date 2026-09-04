use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct OrderId(u64);

impl OrderId {
    pub fn new(value: u64) -> Result<Self, OrderValidationError> {
        (value > 0)
            .then_some(Self(value))
            .ok_or(OrderValidationError::ZeroOrderId)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for OrderId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct Price(u64);

impl Price {
    pub fn new(value: u64) -> Result<Self, OrderValidationError> {
        (value > 0)
            .then_some(Self(value))
            .ok_or(OrderValidationError::ZeroPrice)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct Quantity(u64);

impl Quantity {
    pub fn new(value: u64) -> Result<Self, OrderValidationError> {
        (value > 0)
            .then_some(Self(value))
            .ok_or(OrderValidationError::ZeroQuantity)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    #[default]
    Gtc,
    Ioc,
    Fok,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewOrder {
    pub order_id: OrderId,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
    pub time_in_force: TimeInForce,
    pub post_only: bool,
}

impl NewOrder {
    pub fn new(
        order_id: u64,
        side: Side,
        price: u64,
        quantity: u64,
        time_in_force: TimeInForce,
        post_only: bool,
    ) -> Result<Self, OrderValidationError> {
        if post_only && time_in_force != TimeInForce::Gtc {
            return Err(OrderValidationError::PostOnlyMustBeGtc);
        }

        Ok(Self {
            order_id: OrderId::new(order_id)?,
            side,
            price: Price::new(price)?,
            quantity: Quantity::new(quantity)?,
            time_in_force,
            post_only,
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RestingOrder {
    pub order_id: OrderId,
    pub remaining_quantity: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OrderLocation {
    pub side: Side,
    pub price: Price,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OrderValidationError {
    #[error("order_id must be greater than zero")]
    ZeroOrderId,
    #[error("price must be greater than zero")]
    ZeroPrice,
    #[error("quantity must be greater than zero")]
    ZeroQuantity,
    #[error("post_only orders must use gtc time_in_force")]
    PostOnlyMustBeGtc,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_numeric_values() {
        assert_eq!(OrderId::new(0), Err(OrderValidationError::ZeroOrderId));
        assert_eq!(Price::new(0), Err(OrderValidationError::ZeroPrice));
        assert_eq!(Quantity::new(0), Err(OrderValidationError::ZeroQuantity));
    }

    #[test]
    fn post_only_requires_good_til_cancelled() {
        assert_eq!(
            NewOrder::new(1, Side::Buy, 100, 2, TimeInForce::Ioc, true),
            Err(OrderValidationError::PostOnlyMustBeGtc)
        );
    }
}
