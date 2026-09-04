use serde::Serialize;

use crate::domain::{CancelReport, ExecutionReport, NewOrder, OrderId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderCommand {
    Submit(NewOrder),
    Cancel(OrderId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderCommandResult {
    Submitted(ExecutionReport),
    Cancelled(CancelReport),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "event_type", content = "data", rename_all = "snake_case")]
pub enum DomainEvent {
    OrderProcessed(ExecutionReport),
    OrderCancelled(CancelReport),
}
