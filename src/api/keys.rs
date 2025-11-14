//! Keys API
//!
//! This module provides functionality to manage API keys.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::keys::*;
use uuid::Uuid;

impl RainClient {
    /// Create a key
    ///
    /// # Arguments
    ///
    /// * `request` - The key creation request
    ///
    /// # Returns
    ///
    /// Returns a [`Key`] containing the created key information (including the key itself).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::keys::CreateKeyRequest;
    /// use chrono::Utc;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let request = CreateKeyRequest {
    ///     name: "My API Key".to_string(),
    ///     expires_at: Utc::now() + chrono::Duration::days(90),
    /// };
    /// let key = client.create_key(&request).await?;
    /// println!("Created key: {}", key.key);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_key(&self, request: &CreateKeyRequest) -> Result<Key> {
        let path = "/issuing/keys";
        self.post(path, request).await
    }

    /// Delete a key
    ///
    /// # Arguments
    ///
    /// * `key_id` - The unique identifier of the key to delete
    ///
    /// # Returns
    ///
    /// Returns success (204 No Content) with no response body.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let key_id = Uuid::new_v4();
    /// client.delete_key(&key_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn delete_key(&self, key_id: &Uuid) -> Result<()> {
        let path = format!("/issuing/keys/{key_id}");
        self.delete(&path).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Create a key (blocking)
    #[cfg(feature = "sync")]
    pub fn create_key_blocking(&self, request: &CreateKeyRequest) -> Result<Key> {
        let path = "/issuing/keys";
        self.post_blocking(path, request)
    }

    /// Delete a key (blocking)
    #[cfg(feature = "sync")]
    pub fn delete_key_blocking(&self, key_id: &Uuid) -> Result<()> {
        let path = format!("/issuing/keys/{key_id}");
        self.delete_blocking(&path)
    }
}
