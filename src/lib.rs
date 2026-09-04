//! Rustilleus is a deterministic, in-memory limit order matching service.
//!
//! The crate is split into three layers:
//! - [`domain`] owns exchange rules and contains no HTTP concerns.
//! - [`application`] coordinates commands and publishes domain events.
//! - [`infrastructure`] adapts the application to HTTP, logging, and the runtime.

pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
