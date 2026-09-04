use std::{env, net::SocketAddr};

use thiserror::Error;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_SYMBOL: &str = "BTC-USD";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub symbol: String,
    pub json_logs: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = env::var("APP_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_owned());
        let port = env::var("APP_PORT")
            .map(|value| {
                value
                    .parse::<u16>()
                    .map_err(|_| ConfigError::InvalidPort(value))
            })
            .unwrap_or(Ok(DEFAULT_PORT))?;
        let symbol = env::var("BOOK_SYMBOL").unwrap_or_else(|_| DEFAULT_SYMBOL.to_owned());
        let json_logs = env::var("LOG_FORMAT")
            .map(|value| value.eq_ignore_ascii_case("json"))
            .unwrap_or(true);

        if symbol.trim().is_empty() {
            return Err(ConfigError::EmptySymbol);
        }

        Ok(Self {
            host,
            port,
            symbol,
            json_logs,
        })
    }

    pub fn address(&self) -> Result<SocketAddr, ConfigError> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(ConfigError::InvalidAddress)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("APP_PORT must be an integer between 0 and 65535, got `{0}`")]
    InvalidPort(String),
    #[error("BOOK_SYMBOL cannot be empty")]
    EmptySymbol,
    #[error("invalid listen address: {0}")]
    InvalidAddress(#[source] std::net::AddrParseError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_address_is_valid() {
        let config = Config {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_PORT,
            symbol: DEFAULT_SYMBOL.to_owned(),
            json_logs: true,
        };

        assert_eq!(config.address().unwrap(), "0.0.0.0:8080".parse().unwrap());
    }
}
