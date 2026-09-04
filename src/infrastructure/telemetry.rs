use std::error::Error;

use tracing_subscriber::EnvFilter;

pub fn init(json_logs: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("rustilleus=info,tower_http=info"));

    if json_logs {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(true)
            .json()
            .try_init()?;
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(true)
            .try_init()?;
    }

    Ok(())
}
