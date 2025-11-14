//! Configuration for the Rain SDK
//!
//! This module provides configuration types for the Rain SDK client.
//!
//! # Examples
//!
//! ```no_run
//! use rain_sdk::{Config, Environment};
//!
//! // Use dev environment
//! let config = Config::new(Environment::Dev);
//!
//! // Use production environment
//! let config = Config::new(Environment::Production);
//!
//! // Use custom URL
//! let custom_url = url::Url::parse("https://api.example.com/v1").unwrap();
//! let config = Config::new(Environment::Custom(custom_url));
//!
//! // Configure timeout and user agent
//! let config = Config::new(Environment::Dev)
//!     .with_timeout(60)
//!     .with_user_agent("my-app/1.0".to_string())
//!     .with_logging(true);
//! ```

use url::Url;

/// Environment configuration for the Rain API
///
/// Determines which API endpoint the client will connect to.
///
/// # Examples
///
/// ```no_run
/// use rain_sdk::Environment;
///
/// // Use dev for testing
/// let env = Environment::Dev;
///
/// // Use production
/// let env = Environment::Production;
///
/// // Use custom endpoint
/// let custom_url = url::Url::parse("https://api.example.com/v1").unwrap();
/// let env = Environment::Custom(custom_url);
/// ```
#[derive(Debug, Clone, Default)]
pub enum Environment {
    /// Dev environment
    #[default]
    Dev,
    /// Production environment
    Production,
    /// Custom base URL
    Custom(Url),
}

impl Environment {
    /// Get the base URL for the environment
    ///
    /// Returns the base URL that will be used for API requests.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::Environment;
    ///
    /// let env = Environment::Dev;
    /// let url = env.base_url();
    /// println!("Base URL: {}", url);
    /// ```
    pub fn base_url(&self) -> Url {
        match self {
            Environment::Dev => {
                Url::parse("https://api-dev.raincards.xyz/v1/issuing").expect("Invalid dev URL")
            }
            Environment::Production => {
                Url::parse("https://api.raincards.xyz/v1/issuing").expect("Invalid production URL")
            }
            Environment::Custom(url) => url.clone(),
        }
    }
}

/// Client configuration
///
/// Configuration options for the Rain SDK client, including base URL, timeout,
/// user agent, and logging settings.
///
/// # Examples
///
/// ```no_run
/// use rain_sdk::{Config, Environment};
///
/// // Default configuration
/// let config = Config::new(Environment::Dev);
///
/// // Custom configuration
/// let config = Config::new(Environment::Production)
///     .with_timeout(60)
///     .with_user_agent("my-app/1.0".to_string())
///     .with_logging(true);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Base URL for API requests
    pub base_url: Url,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// User agent string
    pub user_agent: String,
    /// Enable request/response logging
    pub enable_logging: bool,
}

impl Config {
    /// Create a new configuration with default values
    ///
    /// Creates a configuration with:
    /// - Base URL from the environment
    /// - 30 second timeout
    /// - Default user agent (includes SDK version)
    /// - Logging disabled
    ///
    /// # Arguments
    ///
    /// * `environment` - The environment to use (Dev, Production, or Custom)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{Config, Environment};
    ///
    /// let config = Config::new(Environment::Dev);
    /// ```
    pub fn new(environment: Environment) -> Self {
        Self {
            base_url: environment.base_url(),
            timeout_secs: 30,
            user_agent: format!("rain-sdk/{}", env!("CARGO_PKG_VERSION")),
            enable_logging: false,
        }
    }

    /// Set the request timeout in seconds
    ///
    /// # Arguments
    ///
    /// * `timeout_secs` - Timeout in seconds
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{Config, Environment};
    ///
    /// let config = Config::new(Environment::Dev)
    ///     .with_timeout(60); // 60 second timeout
    /// ```
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Set a custom user agent string
    ///
    /// # Arguments
    ///
    /// * `user_agent` - Custom user agent string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{Config, Environment};
    ///
    /// let config = Config::new(Environment::Dev)
    ///     .with_user_agent("my-app/1.0".to_string());
    /// ```
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    /// Enable or disable request/response logging
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable logging
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{Config, Environment};
    ///
    /// let config = Config::new(Environment::Dev)
    ///     .with_logging(true);
    /// ```
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.enable_logging = enable;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(Environment::default())
    }
}
