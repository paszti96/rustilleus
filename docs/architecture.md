# Architecture

Rustilleus uses a ports-and-adapters layout so exchange rules are independent of
HTTP and framework code.

```text
HTTP request
    │
    ▼
infrastructure/http.rs       JSON DTOs, validation mapping, status codes
    │ OrderCommand
    ▼
application/service.rs       transaction boundary, locking, event publication
    │ MatchingEngine trait
    ▼
domain/book.rs               price-time matching and order-book invariants
    │ DomainEvent
    ▼
infrastructure/events.rs     structured event log (replaceable event sink)
```

## Design patterns

- **Ports and adapters / hexagonal architecture:** `MatchingEngine` and
  `EventSink` are ports. The in-memory book, HTTP handlers, and tracing sink are
  adapters. Exchange rules can be tested without starting a server.
- **Facade:** `OrderBookService` gives transport code one small interface and owns
  synchronization and event publication.
- **Command:** `OrderCommand` represents submit and cancel use cases as values.
- **Observer:** `EventSink` receives successful domain events. A Kafka or database
  adapter can replace the logging implementation without changing the book.
- **Strategy / dependency inversion:** the service depends on the
  `MatchingEngine` trait, not the concrete `OrderBook` implementation.
- **Newtype:** `OrderId`, `TradeId`, `Price`, and `Quantity` prevent accidental
  mixing of values that share the same underlying integer representation.

## Matching invariants

1. The highest bid and lowest ask always have execution priority.
2. Orders at the same price execute first-in, first-out.
3. Trades execute at the resting maker order's price.
4. A resting order appears in exactly one price queue and the active-order index.
5. Fully filled and cancelled orders are removed from both structures.
6. An order ID cannot be reused during the process lifetime.
7. FOK validation and matching are atomic under the service lock.
8. Prices and quantities are integers, avoiding floating-point rounding errors.

## Data structures and complexity

| Operation | Structure | Complexity |
| --- | --- | --- |
| Best bid/ask lookup | `BTreeMap` | O(log P) lookup and update |
| FIFO within one price | `VecDeque` | O(1) front/pop/push |
| Duplicate/order lookup | `HashMap` / `HashSet` | Average O(1) |
| Cancel | indexed price lookup + level scan | O(log P + L) |

`P` is the number of price levels and `L` is the number of orders at one price.
For extremely deep levels, an intrusive linked queue plus direct node handles
would make cancellation O(1), at the cost of considerably more complexity.

## Concurrency model

One mutex protects one deterministic matching engine. This is intentional:
commands for a single symbol must be totally ordered, and a single writer avoids
races and lock-ordering bugs inside the hot path. Scale horizontally by assigning
different symbols to different engine instances or partitions, not by processing
one symbol concurrently.

## Production evolution

The current service is an honest single-symbol, in-memory engine. Before handling
real money, add an append-only command journal, recovery snapshots, authentication,
risk checks, rate limits, market-data streaming, and a sequenced durable event bus.
Those concerns belong in new adapters around the deterministic domain core.
