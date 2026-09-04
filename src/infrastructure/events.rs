use crate::application::{DomainEvent, EventSink};

#[derive(Debug, Default)]
pub struct TracingEventSink;

impl EventSink for TracingEventSink {
    fn publish(&self, event: &DomainEvent) {
        tracing::info!(?event, "domain event published");
    }
}
