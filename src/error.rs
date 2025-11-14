//! Error types for the Rain SDK

use serde::{Deserialize, Serialize};
use std::fmt;

/// Main error type for the Rain SDK
#[derive(Debug, thiserror::Error)]
pub enum RainError {
    /// HTTP client errors
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// API error responses from the server
    #[error("API error: {0}")]
    ApiError(Box<ApiErrorResponse>),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    AuthError(String),

    /// Request validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// JSON deserialization errors
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),

    /// Other errors
    #[error("Error: {0}")]
    Other(#[from] anyhow::Error),
}

/// API error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    /// Error message
    pub message: Option<String>,

    /// Error code
    pub code: Option<String>,

    /// Error details
    pub details: Option<serde_json::Value>,
}

impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref message) = self.message {
            write!(f, "{message}")?;
        } else if let Some(ref code) = self.code {
            write!(f, "{code}")?;
        } else {
            write!(f, "API error")?;
        }
        Ok(())
    }
}

impl std::error::Error for ApiErrorResponse {}

impl From<ApiErrorResponse> for RainError {
    fn from(err: ApiErrorResponse) -> Self {
        RainError::ApiError(Box::new(err))
    }
}

/// Result type alias for Rain SDK operations
pub type Result<T> = std::result::Result<T, RainError>;

impl From<serde_urlencoded::ser::Error> for RainError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        RainError::ValidationError(format!("URL encoding error: {err}"))
    }
}
