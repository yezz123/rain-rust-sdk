//! Authentication module for API key management
//!
//! This module provides authentication functionality for the Rain SDK.
//! It supports API key authentication via the `Api-Key` header.
//!
//! # Authentication Methods
//!
//! ## API Key Authentication
//!
//! Simple authentication using an API key:
//!
//! ```no_run
//! use rain_sdk::AuthConfig;
//!
//! let auth = AuthConfig::with_api_key("your-api-key".to_string());
//! ```

/// Authentication configuration
///
/// Configures how the client authenticates with the Rain API.
/// Supports API key authentication via the `Api-Key` header.
///
/// # Examples
///
/// ```no_run
/// use rain_sdk::AuthConfig;
///
/// // API key authentication
/// let auth = AuthConfig::with_api_key("your-api-key".to_string());
/// ```
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// API key for Api-Key header
    pub api_key: String,
}

impl AuthConfig {
    /// Create new auth config with API key
    ///
    /// This method uses simple API key authentication via the `Api-Key` header.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Rain API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::AuthConfig;
    ///
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// ```
    pub fn with_api_key(api_key: String) -> Self {
        Self { api_key }
    }
}

/// Add authentication headers to a request builder
#[cfg(feature = "async")]
pub fn add_auth_headers_async(
    builder: reqwest::RequestBuilder,
    auth_config: &AuthConfig,
) -> reqwest::RequestBuilder {
    builder.header("Api-Key", &auth_config.api_key)
}

/// Add authentication headers to a request builder (blocking)
#[cfg(feature = "sync")]
pub fn add_auth_headers_sync(
    builder: reqwest::blocking::RequestBuilder,
    auth_config: &AuthConfig,
) -> reqwest::blocking::RequestBuilder {
    builder.header("Api-Key", &auth_config.api_key)
}
