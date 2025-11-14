//! # Rain SDK
//!
//! A modern, type-safe Rust SDK for the Rain Cards API.
//!
//! ## Features
//!
//! - **Async and Sync Support**: Use async/await or blocking operations
//! - **Type Safety**: Strongly typed models for all API endpoints
//! - **API Key Authentication**: Simple API key-based authentication
//! - **Comprehensive Error Handling**: Detailed error types with context
//!
//! ## Quick Start
//!
//! ```no_run
//! use rain_sdk::{RainClient, Config, Environment, AuthConfig};
//!
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::new(Environment::Dev);
//! let auth = AuthConfig::with_api_key("your-api-key".to_string());
//! let client = RainClient::new(config, auth)?;
//!
//! // Use the client to make API calls
//! # Ok(())
//! # }
//! ```
//!
//! ## Documentation
//!
//! See the [documentation](https://docs.rs/rain-sdk) for detailed API reference.

pub mod api;
pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod models;

pub use auth::AuthConfig;
pub use client::RainClient;
pub use config::{Config, Environment};
pub use error::{RainError, Result};

// Re-export API modules
pub use api::{
    applications, balances, cards, companies, contracts, disputes, keys, payments, reports,
    shipping_groups, signatures, subtenants, transactions, users, webhooks,
};
