//! Use-case orchestration and ports connecting the domain to infrastructure.

mod command;
mod service;

pub use command::{DomainEvent, OrderCommand, OrderCommandResult};
pub use service::{EventSink, OrderBookService, ServiceError};
