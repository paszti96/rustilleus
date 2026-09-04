use std::{error::Error, sync::Arc};

use rustilleus::{
    application::OrderBookService,
    config::Config,
    domain::OrderBook,
    infrastructure::{events::TracingEventSink, http, telemetry},
};
use tokio::{net::TcpListener, signal};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = Config::from_env()?;
    telemetry::init(config.json_logs)?;

    let address = config.address()?;
    let symbol = config.symbol.clone();
    let service = OrderBookService::new(
        Box::new(OrderBook::new(config.symbol)),
        Arc::new(TracingEventSink),
    );
    let app = http::router(service);
    let listener = TcpListener::bind(address).await?;

    info!(%address, %symbol, "matching service started");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    info!("matching service stopped");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
