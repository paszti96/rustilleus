# Rustilleus

[![CI](https://github.com/paszti96/rustilleus/actions/workflows/ci.yml/badge.svg)](https://github.com/paszti96/rustilleus/actions/workflows/ci.yml)
[![Publish container](https://github.com/paszti96/rustilleus/actions/workflows/deploy.yml/badge.svg)](https://github.com/paszti96/rustilleus/actions/workflows/deploy.yml)

Rustilleus is a deterministic, production-oriented limit order matching service
written in Rust. It demonstrates exchange-domain modeling, price-time priority,
clean architecture, concurrency control, structured errors and observability,
API design, automated tests, and container delivery.

The earlier educational project remains available on the `tutorial` branch.

## What it supports

- Buy and sell limit orders
- Price priority, then FIFO time priority at each price
- GTC (good-til-cancelled), IOC (immediate-or-cancel), and FOK (fill-or-kill)
- Post-only orders that reject rather than remove liquidity
- Partial fills across multiple price levels
- Cancellation of active orders
- Aggregated, depth-limited book snapshots
- Duplicate order-ID protection for the process lifetime
- Integer prices and quantities, avoiding floating-point money errors
- Request IDs, body limits, request timeouts, panic containment, and tracing
- Graceful shutdown on Ctrl+C or SIGTERM

## Quick start

```bash
cargo run
```

The service listens on `http://localhost:8080` and manages `BTC-USD` by default.
Open another terminal and check it:

```bash
curl http://localhost:8080/health
```

Add a resting sell order. Prices are integer ticks; for a currency with two
decimal places, `10000` can represent `100.00`:

```bash
curl -i -X POST http://localhost:8080/v1/orders \
  -H 'content-type: application/json' \
  -d '{
    "order_id": 1,
    "side": "sell",
    "price": 10000,
    "quantity": 5,
    "time_in_force": "gtc",
    "post_only": false
  }'
```

Submit an aggressive buy that matches the resting sell:

```bash
curl -i -X POST http://localhost:8080/v1/orders \
  -H 'content-type: application/json' \
  -d '{
    "order_id": 2,
    "side": "buy",
    "price": 10100,
    "quantity": 3
  }'
```

Inspect or cancel:

```bash
curl 'http://localhost:8080/v1/orderbook?depth=20'
curl -i -X DELETE http://localhost:8080/v1/orders/1
```

## Matching semantics

An incoming buy may trade with asks priced at or below its limit. An incoming
sell may trade with bids priced at or above its limit. The best price is selected
first; orders at that price execute in arrival order. Every trade uses the price
of the older resting order (the maker), as a real exchange normally does.

| Time in force | Behavior |
| --- | --- |
| `gtc` | Match immediately, then keep any remainder on the book |
| `ioc` | Match immediately, then cancel any remainder |
| `fok` | Fill the complete quantity immediately or execute nothing |

A post-only order must be GTC. If it would trade immediately, the API returns
`409 Conflict`, allowing a market maker to guarantee that it adds liquidity.

## API

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/health` | Readiness and version information |
| `GET` | `/v1/orderbook?depth=20` | Aggregated best-first price levels |
| `POST` | `/v1/orders` | Validate, match, and possibly rest an order |
| `DELETE` | `/v1/orders/{order_id}` | Cancel an active resting order |

The full contract is in [docs/openapi.yaml](docs/openapi.yaml). Errors are JSON
objects with stable `code` and human-readable `message` fields.

## Architecture and design patterns

```text
HTTP adapter → application facade → MatchingEngine port → order-book domain
                         │
                         └→ EventSink observer → structured tracing adapter
```

The code uses ports and adapters, Facade, Command, Observer, Strategy/dependency
inversion, and Newtype patterns. Rust does not have class inheritance; structs
encapsulate state, `impl` blocks provide behavior, traits provide interfaces and
polymorphism, and composition connects objects explicitly.

See [docs/architecture.md](docs/architecture.md) for invariants, complexity,
concurrency decisions, and the path from this portfolio service to an exchange
that could handle real money.

## Project layout

```text
src/
├── domain/          Orders, trades, book data structures, matching rules
├── application/     Commands, service facade, interfaces, domain events
├── infrastructure/  Axum HTTP API, tracing, event adapter
├── config.rs        Environment-based startup configuration
├── lib.rs           Public library surface
└── main.rs          Runtime composition and graceful shutdown
tests/               Black-box matching scenarios through public APIs
docs/                Architecture decisions and OpenAPI contract
.github/             CI, container publishing, dependency updates
```

The domain layer does not import Axum or Tokio. That separation keeps the most
important business rules deterministic, fast, and easy to test.

## Configuration

| Variable | Default | Description |
| --- | --- | --- |
| `APP_HOST` | `0.0.0.0` | Interface on which the HTTP server listens |
| `APP_PORT` | `8080` | HTTP port |
| `BOOK_SYMBOL` | `BTC-USD` | Symbol managed by this engine instance |
| `LOG_FORMAT` | `json` | Use `pretty` for human-readable local logs |
| `RUST_LOG` | `rustilleus=info,tower_http=info` | Tracing filter |

Copy `.env.example` values into your shell or container environment as needed.
The application intentionally does not read `.env` files by itself, keeping
production secret/configuration injection explicit.

## Quality checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features --locked
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --locked
```

Tests cover validation, price priority, FIFO priority, multi-level fills, partial
fills, IOC, FOK, post-only rejection, cancellation, duplicate IDs, snapshots,
event publication, HTTP responses, and request IDs.

Run the deterministic matching throughput harness in optimized mode:

```bash
cargo run --release --example throughput
```

It rests 100,000 sell orders and then matches them with 100,000 buys, reporting
elapsed time and orders per second. Treat the result as a local engineering signal,
not a universal claim: hardware, compiler version, and build settings all matter.

## Container and delivery

Build and run the same minimal, non-root container used by CI:

```bash
docker build -t rustilleus .
docker run --rm -p 8080:8080 rustilleus
```

GitHub Actions provides three pipelines:

- `ci.yml` formats, lints, tests, builds docs, and verifies the production image
  for pushes and pull requests.
- `deploy.yml` publishes provenance-attested, SBOM-enabled images to GitHub
  Container Registry on every push to `main`, on `v*` tags, or by manual dispatch.
- `security.yml` checks dependencies against the RustSec advisory database when
  dependencies change and every Monday.

Published images use tags such as `ghcr.io/paszti96/rustilleus:main` and a commit
SHA tag. Dependabot checks Cargo, Actions, and Docker dependencies weekly.

## Scope and production trade-offs

This service makes its limits explicit. State is in memory and one process owns
one symbol. That is a sound core for demonstrations, simulations, and further
engineering, but it is not yet suitable for real funds. A real venue additionally
needs durable command journaling and replay, authentication, pre-trade risk checks,
rate limiting, market-data feeds, audit retention, multi-symbol partitioning,
operational metrics, and extensive performance and failure testing.
