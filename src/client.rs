//! Main HTTP client for the Rain SDK
//!
//! This module provides the core HTTP client for making requests to the Rain API.
//! The client supports both async and blocking (synchronous) operations.
//!
//! # Features
//!
//! - **Async Support**: Use `async`/`await` for non-blocking operations
//! - **Blocking Support**: Use synchronous methods for simpler code
//! - **Automatic Authentication**: Handles API key authentication
//! - **Error Handling**: Comprehensive error types with detailed context
//!
//! # Examples
//!
//! ## Async Client
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
//! // Use async methods
//! # Ok(())
//! # }
//! ```
//!
//! ## Blocking Client
//!
//! ```no_run
//! use rain_sdk::{RainClient, Config, Environment, AuthConfig};
//!
//! # #[cfg(feature = "sync")]
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::new(Environment::Dev);
//! let auth = AuthConfig::with_api_key("your-api-key".to_string());
//! let client = RainClient::new(config, auth)?;
//!
//! // Use blocking methods
//! # Ok(())
//! # }
//! ```

use crate::auth::AuthConfig;
use crate::config::Config;
use crate::error::{RainError, Result};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use url::Url;

/// Main client for interacting with the Rain API
///
/// This client provides methods to interact with all Rain API endpoints.
/// It handles authentication, request building, and response parsing automatically.
///
/// # Thread Safety
///
/// The client is `Clone` and can be shared across threads safely.
///
/// # Examples
///
/// ```no_run
/// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
///
/// # #[cfg(feature = "async")]
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = Config::new(Environment::Dev);
/// let auth = AuthConfig::with_api_key("your-api-key".to_string());
/// let client = RainClient::new(config, auth)?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct RainClient {
    config: Config,
    auth_config: AuthConfig,
    #[cfg(feature = "async")]
    client: reqwest::Client,
    #[cfg(feature = "sync")]
    blocking_client: reqwest::blocking::Client,
}

impl RainClient {
    /// Create a new client with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Client configuration (environment, timeout, etc.)
    /// * `auth_config` - Authentication configuration (API key)
    ///
    /// # Returns
    ///
    /// Returns a new `RainClient` instance ready to make API requests.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP client cannot be created
    /// - The user agent string is invalid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(config: Config, auth_config: AuthConfig) -> Result<Self> {
        #[cfg(feature = "async")]
        let client = {
            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(
                "User-Agent",
                HeaderValue::from_str(&config.user_agent)
                    .map_err(|e| RainError::Other(anyhow::anyhow!("Invalid user agent: {e}")))?,
            );

            reqwest::Client::builder()
                .default_headers(headers)
                .timeout(std::time::Duration::from_secs(config.timeout_secs))
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()
                .map_err(RainError::HttpError)?
        };

        #[cfg(feature = "sync")]
        let blocking_client = {
            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(
                "User-Agent",
                HeaderValue::from_str(&config.user_agent)
                    .map_err(|e| RainError::Other(anyhow::anyhow!("Invalid user agent: {e}")))?,
            );

            reqwest::blocking::Client::builder()
                .default_headers(headers)
                .timeout(std::time::Duration::from_secs(config.timeout_secs))
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()
                .map_err(|e| {
                    RainError::Other(anyhow::anyhow!("Failed to create blocking client: {e}"))
                })?
        };

        Ok(Self {
            config,
            auth_config,
            #[cfg(feature = "async")]
            client,
            #[cfg(feature = "sync")]
            blocking_client,
        })
    }

    /// Get the base URL for API requests
    ///
    /// Returns the base URL that all API requests will be made against.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let base_url = client.base_url();
    /// println!("API base URL: {}", base_url);
    /// # Ok(())
    /// # }
    /// ```
    pub fn base_url(&self) -> &Url {
        &self.config.base_url
    }

    /// Build a full URL from a path
    fn build_url(&self, path: &str) -> Result<Url> {
        // If path starts with /, we need to preserve the base URL's path
        let path_to_join = path.strip_prefix('/').unwrap_or(path);

        let mut url = self.config.base_url.clone();
        url.path_segments_mut()
            .map_err(|_| RainError::Other(anyhow::anyhow!("Cannot be a base URL")))?
            .pop_if_empty()
            .extend(path_to_join.split('/').filter(|s| !s.is_empty()));

        Ok(url)
    }

    #[cfg(feature = "async")]
    /// Make an async GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.build_url(path)?;
        let builder = self.client.get(url.as_str());
        let builder = crate::auth::add_auth_headers_async(builder, &self.auth_config);

        let response = builder.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "async")]
    /// Make an async GET request and return raw bytes
    pub async fn get_bytes(&self, path: &str) -> Result<Vec<u8>> {
        let url = self.build_url(path)?;
        let builder = self.client.get(url.as_str());
        let builder = crate::auth::add_auth_headers_async(builder, &self.auth_config);

        let response = builder.send().await?;
        let status = response.status();
        if status.is_success() {
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        } else {
            let text = response.text().await?;
            Err(RainError::Other(anyhow::anyhow!("HTTP {status}: {text}")))
        }
    }

    #[cfg(feature = "async")]
    /// Make an async POST request
    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let builder = self.client.post(url.as_str()).body(body_bytes.clone());
        let builder = crate::auth::add_auth_headers_async(builder, &self.auth_config);

        let response = builder.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "async")]
    /// Make an async PATCH request
    pub async fn patch<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let builder = self.client.patch(url.as_str()).body(body_bytes.clone());
        let builder = crate::auth::add_auth_headers_async(builder, &self.auth_config);

        let response = builder.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "async")]
    /// Make an async PUT request
    pub async fn put<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let builder = self.client.put(url.as_str()).body(body_bytes.clone());
        let builder = crate::auth::add_auth_headers_async(builder, &self.auth_config);

        let response = builder.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "async")]
    /// Make an async GET request with custom headers
    pub async fn get_with_headers<T: DeserializeOwned>(
        &self,
        path: &str,
        headers: Vec<(&str, &str)>,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let mut builder = self.client.get(url.as_str());
        builder = crate::auth::add_auth_headers_async(builder, &self.auth_config);

        for (key, value) in headers {
            builder = builder.header(key, value);
        }

        let response = builder.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "async")]
    /// Make an async PUT request with custom headers
    pub async fn put_with_headers<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
        headers: Vec<(&str, &str)>,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let mut builder = self.client.put(url.as_str()).body(body_bytes.clone());
        builder = crate::auth::add_auth_headers_async(builder, &self.auth_config);

        for (key, value) in headers {
            builder = builder.header(key, value);
        }

        let response = builder.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "async")]
    /// Make an async DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = self.build_url(path)?;
        let builder = self.client.delete(url.as_str());
        let builder = crate::auth::add_auth_headers_async(builder, &self.auth_config);

        let response = builder.send().await?;
        let status = response.status();
        if status.is_success() || status == reqwest::StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let text = response.text().await?;
            Err(RainError::Other(anyhow::anyhow!("HTTP {status}: {text}")))
        }
    }

    #[cfg(feature = "async")]
    /// Make an async PUT request with multipart form data
    pub async fn put_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let mut headers = HeaderMap::new();
        // Don't set Content-Type for multipart - reqwest will set it with boundary
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            "User-Agent",
            HeaderValue::from_str(&self.config.user_agent)
                .map_err(|e| RainError::Other(anyhow::anyhow!("Invalid user agent: {e}")))?,
        );

        let request = self
            .client
            .put(url.as_str())
            .headers(headers)
            .header("Api-Key", &self.auth_config.api_key)
            .multipart(form);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "async")]
    /// Make an async PUT request with multipart form data that returns nothing (204)
    pub async fn put_multipart_no_content(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<()> {
        let url = self.build_url(path)?;
        let mut headers = HeaderMap::new();
        // Don't set Content-Type for multipart - reqwest will set it with boundary
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            "User-Agent",
            HeaderValue::from_str(&self.config.user_agent)
                .map_err(|e| RainError::Other(anyhow::anyhow!("Invalid user agent: {e}")))?,
        );

        let request = self
            .client
            .put(url.as_str())
            .headers(headers)
            .header("Api-Key", &self.auth_config.api_key)
            .multipart(form);

        let response = request.send().await?;
        let status = response.status();
        if status == reqwest::StatusCode::NO_CONTENT || status.is_success() {
            Ok(())
        } else {
            let text = response.text().await?;
            Err(RainError::Other(anyhow::anyhow!("HTTP {status}: {text}")))
        }
    }

    #[cfg(feature = "sync")]
    /// Make a blocking GET request
    pub fn get_blocking<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.build_url(path)?;
        let builder = self.blocking_client.get(url.as_str());
        let builder = crate::auth::add_auth_headers_sync(builder, &self.auth_config);

        let response = builder.send()?;
        self.handle_blocking_response(response)
    }

    #[cfg(feature = "sync")]
    /// Make a blocking GET request and return raw bytes
    pub fn get_bytes_blocking(&self, path: &str) -> Result<Vec<u8>> {
        let url = self.build_url(path)?;
        let builder = self.blocking_client.get(url.as_str());
        let builder = crate::auth::add_auth_headers_sync(builder, &self.auth_config);

        let response = builder.send()?;
        let status = response.status();
        if status.is_success() {
            let bytes = response.bytes()?;
            Ok(bytes.to_vec())
        } else {
            let text = response.text()?;
            Err(RainError::Other(anyhow::anyhow!("HTTP {status}: {text}")))
        }
    }

    #[cfg(feature = "sync")]
    /// Make a blocking POST request
    pub fn post_blocking<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let builder = self
            .blocking_client
            .post(url.as_str())
            .body(body_bytes.clone());
        let builder = crate::auth::add_auth_headers_sync(builder, &self.auth_config);

        let response = builder.send()?;
        self.handle_blocking_response(response)
    }

    #[cfg(feature = "sync")]
    /// Make a blocking PATCH request
    pub fn patch_blocking<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let builder = self
            .blocking_client
            .patch(url.as_str())
            .body(body_bytes.clone());
        let builder = crate::auth::add_auth_headers_sync(builder, &self.auth_config);

        let response = builder.send()?;
        self.handle_blocking_response(response)
    }

    #[cfg(feature = "sync")]
    /// Make a blocking PUT request
    pub fn put_blocking<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let builder = self
            .blocking_client
            .put(url.as_str())
            .body(body_bytes.clone());
        let builder = crate::auth::add_auth_headers_sync(builder, &self.auth_config);

        let response = builder.send()?;
        self.handle_blocking_response(response)
    }

    #[cfg(feature = "sync")]
    /// Make a blocking PUT request with multipart form data that returns nothing (204)
    pub fn put_multipart_blocking_no_content(
        &self,
        path: &str,
        form: reqwest::blocking::multipart::Form,
    ) -> Result<()> {
        let url = self.build_url(path)?;
        use reqwest::blocking::header::{HeaderMap, HeaderValue, ACCEPT};
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            "User-Agent",
            HeaderValue::from_str(&self.config.user_agent)
                .map_err(|e| RainError::Other(anyhow::anyhow!("Invalid user agent: {e}")))?,
        );

        let response = self
            .blocking_client
            .put(url.as_str())
            .headers(headers)
            .header("Api-Key", &self.auth_config.api_key)
            .multipart(form)
            .send()?;

        let status = response.status();
        if status == reqwest::StatusCode::NO_CONTENT || status.is_success() {
            Ok(())
        } else {
            let text = response.text()?;
            Err(RainError::Other(anyhow::anyhow!("HTTP {status}: {text}")))
        }
    }

    #[cfg(feature = "sync")]
    /// Make a blocking DELETE request
    pub fn delete_blocking(&self, path: &str) -> Result<()> {
        let url = self.build_url(path)?;
        let builder = self.blocking_client.delete(url.as_str());
        let builder = crate::auth::add_auth_headers_sync(builder, &self.auth_config);

        let response = builder.send()?;
        let status = response.status();
        if status.is_success() || status == reqwest::StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let text = response.text()?;
            Err(RainError::Other(anyhow::anyhow!("HTTP {status}: {text}")))
        }
    }

    #[cfg(feature = "async")]
    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();
        let url = response.url().clone();
        let text = response.text().await?;

        // Handle 202 Accepted (typically has no body)
        if status == reqwest::StatusCode::ACCEPTED {
            if text.is_empty() {
                // Try to deserialize as empty JSON object for 202
                serde_json::from_str("{}")
                    .or_else(|_| serde_json::from_str("null"))
                    .map_err(|_| RainError::ValidationError("Empty response body".to_string()))
            } else {
                serde_json::from_str(&text).map_err(RainError::DeserializationError)
            }
        } else if status.is_success() {
            if text.is_empty() {
                // Handle 204 No Content
                serde_json::from_str("null")
                    .map_err(|_| RainError::ValidationError("Empty response body".to_string()))
            } else {
                serde_json::from_str(&text).map_err(RainError::DeserializationError)
            }
        } else {
            // Try to parse as error response
            match serde_json::from_str::<crate::error::ApiErrorResponse>(&text) {
                Ok(api_error) => Err(RainError::ApiError {
                    status: status.as_u16(),
                    response: Box::new(api_error),
                }),
                Err(_) => Err(RainError::Other(anyhow::anyhow!(
                    "HTTP {} from {}: {}",
                    status,
                    url,
                    if text.len() > 200 {
                        format!("{}...", &text[..200])
                    } else {
                        text
                    }
                ))),
            }
        }
    }

    #[cfg(feature = "sync")]
    fn handle_blocking_response<T: DeserializeOwned>(
        &self,
        response: reqwest::blocking::Response,
    ) -> Result<T> {
        let status = response.status();
        let text = response.text()?;

        // Handle 202 Accepted (typically has no body)
        if status == reqwest::StatusCode::ACCEPTED {
            if text.is_empty() {
                // Try to deserialize as empty JSON object for 202
                serde_json::from_str("{}")
                    .or_else(|_| serde_json::from_str("null"))
                    .map_err(|_| RainError::ValidationError("Empty response body".to_string()))
            } else {
                serde_json::from_str(&text).map_err(RainError::DeserializationError)
            }
        } else if status.is_success() {
            if text.is_empty() {
                // Handle 204 No Content
                serde_json::from_str("null")
                    .map_err(|_| RainError::ValidationError("Empty response body".to_string()))
            } else {
                serde_json::from_str(&text).map_err(RainError::DeserializationError)
            }
        } else {
            // Try to parse as error response
            match serde_json::from_str::<crate::error::ApiErrorResponse>(&text) {
                Ok(api_error) => Err(RainError::ApiError {
                    status: status.as_u16(),
                    response: Box::new(api_error),
                }),
                Err(_) => Err(RainError::Other(anyhow::anyhow!("HTTP {status}: {text}"))),
            }
        }
    }
}
