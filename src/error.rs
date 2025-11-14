//! Error types for the Rain SDK
//!
//! This module defines all error types used throughout the Rain SDK, including
//! HTTP errors, API errors, and validation errors.
//!
//! # API Error Codes
//!
//! The Rain API uses the following HTTP status codes for errors:
//!
//! - `400` - Invalid request (bad request parameters or body)
//! - `401` - Invalid authorization / Unauthorized (missing or invalid API key)
//! - `403` - Forbidden (insufficient permissions)
//! - `404` - Not found (resource not found: User, Card, Company, Transaction, Team, etc.)
//! - `409` - Conflict (e.g., "Company already has a contract on this chain", "User already has a contract on this chain", "Another active signature already exists")
//! - `423` - Locked (e.g., "User address is locked")
//! - `500` - Internal server error

use serde::{Deserialize, Serialize};
use std::fmt;

/// Main error type for the Rain SDK
#[derive(Debug, thiserror::Error)]
pub enum RainError {
    /// HTTP client errors
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// API error responses from the server
    ///
    /// Contains the HTTP status code and error details from the API.
    /// Common status codes:
    /// - 400: Invalid request
    /// - 401: Invalid authorization / Unauthorized
    /// - 403: Forbidden
    /// - 404: Not found
    /// - 409: Conflict
    /// - 423: Locked
    /// - 500: Internal server error
    #[error("API error (status {status}): {response}")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error response details
        response: Box<ApiErrorResponse>,
    },

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
///
/// This structure represents error responses from the Rain API.
/// The API may return different error formats, so all fields are optional.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    /// Error message describing what went wrong
    pub message: Option<String>,

    /// Error code (may be a string identifier or numeric code)
    pub code: Option<String>,

    /// Additional error details (structured data)
    pub details: Option<serde_json::Value>,
}

impl ApiErrorResponse {
    /// Create a new API error response with a message
    pub fn new(message: String) -> Self {
        Self {
            message: Some(message),
            code: None,
            details: None,
        }
    }

    /// Create a new API error response with a message and code
    pub fn with_code(message: String, code: String) -> Self {
        Self {
            message: Some(message),
            code: Some(code),
            details: None,
        }
    }
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
        // Default to 500 if status is not available
        RainError::ApiError {
            status: 500,
            response: Box::new(err),
        }
    }
}

/// Result type alias for Rain SDK operations
pub type Result<T> = std::result::Result<T, RainError>;

impl From<serde_urlencoded::ser::Error> for RainError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        RainError::ValidationError(format!("URL encoding error: {err}"))
    }
}
