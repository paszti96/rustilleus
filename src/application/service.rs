use std::sync::{Arc, Mutex};

use thiserror::Error;

use crate::domain::{BookError, BookSnapshot, MatchingEngine};

use super::{DomainEvent, OrderCommand, OrderCommandResult};

pub trait EventSink: Send + Sync {
    fn publish(&self, event: &DomainEvent);
}

#[derive(Clone)]
pub struct OrderBookService {
    engine: Arc<Mutex<Box<dyn MatchingEngine>>>,
    event_sink: Arc<dyn EventSink>,
}

impl OrderBookService {
    pub fn new(engine: Box<dyn MatchingEngine>, event_sink: Arc<dyn EventSink>) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            event_sink,
        }
    }

    pub fn execute(&self, command: OrderCommand) -> Result<OrderCommandResult, ServiceError> {
        let result = {
            let mut engine = self
                .engine
                .lock()
                .map_err(|_| ServiceError::EngineUnavailable)?;

            match command {
                OrderCommand::Submit(order) => engine
                    .submit(order)
                    .map(OrderCommandResult::Submitted)
                    .map_err(ServiceError::from),
                OrderCommand::Cancel(order_id) => engine
                    .cancel(order_id)
                    .map(OrderCommandResult::Cancelled)
                    .map_err(ServiceError::from),
            }
        }?;

        let event = match &result {
            OrderCommandResult::Submitted(report) => DomainEvent::OrderProcessed(report.clone()),
            OrderCommandResult::Cancelled(report) => DomainEvent::OrderCancelled(report.clone()),
        };
        self.event_sink.publish(&event);
        Ok(result)
    }

    pub fn snapshot(&self, depth: usize) -> Result<BookSnapshot, ServiceError> {
        let engine = self
            .engine
            .lock()
            .map_err(|_| ServiceError::EngineUnavailable)?;
        Ok(engine.snapshot(depth))
    }
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Book(#[from] BookError),
    #[error("matching engine is temporarily unavailable")]
    EngineUnavailable,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{NewOrder, OrderBook, Side, TimeInForce};

    #[derive(Default)]
    struct RecordingEventSink {
        events: Mutex<Vec<DomainEvent>>,
    }

    impl EventSink for RecordingEventSink {
        fn publish(&self, event: &DomainEvent) {
            self.events.lock().unwrap().push(event.clone());
        }
    }

    #[test]
    fn command_publishes_an_event_after_success() {
        let sink = Arc::new(RecordingEventSink::default());
        let service = OrderBookService::new(Box::new(OrderBook::new("TEST")), sink.clone());
        let order = NewOrder::new(1, Side::Buy, 100, 2, TimeInForce::Gtc, false).unwrap();

        let result = service.execute(OrderCommand::Submit(order)).unwrap();

        assert!(matches!(result, OrderCommandResult::Submitted(_)));
        assert_eq!(sink.events.lock().unwrap().len(), 1);
    }

    #[test]
    fn failed_command_does_not_publish_an_event() {
        let sink = Arc::new(RecordingEventSink::default());
        let service = OrderBookService::new(Box::new(OrderBook::new("TEST")), sink.clone());
        let missing_id = crate::domain::OrderId::new(404).unwrap();

        let result = service.execute(OrderCommand::Cancel(missing_id));

        assert!(matches!(
            result,
            Err(ServiceError::Book(BookError::OrderNotFound(_)))
        ));
        assert!(sink.events.lock().unwrap().is_empty());
    }
}
