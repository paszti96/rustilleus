FROM rust:1.98-slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && printf 'fn main() {}\n' > src/main.rs && cargo build --release --locked

COPY src ./src
RUN touch src/main.rs && cargo build --release --locked

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime

COPY --from=builder /app/target/release/rustilleus /usr/local/bin/rustilleus
ENV APP_HOST=0.0.0.0 \
    APP_PORT=8080 \
    BOOK_SYMBOL=BTC-USD \
    LOG_FORMAT=json \
    RUST_LOG=rustilleus=info,tower_http=info
EXPOSE 8080
USER nonroot:nonroot
ENTRYPOINT ["/usr/local/bin/rustilleus"]
