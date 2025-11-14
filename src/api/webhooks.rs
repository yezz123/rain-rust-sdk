//! Webhooks API
//!
//! This module provides functionality to manage webhooks.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::webhooks::*;
use uuid::Uuid;

impl RainClient {
    /// Get all webhooks
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters to filter webhooks
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Webhook>`] containing the list of webhooks.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `401` - Invalid authorization
    /// - `403` - Forbidden
    /// - `404` - User not found
    /// - `500` - Internal server error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::webhooks::ListWebhooksParams;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let params = ListWebhooksParams {
    ///     resource_id: None,
    ///     resource_type: None,
    ///     resource_action: None,
    ///     request_sent_at_before: None,
    ///     request_sent_at_after: None,
    ///     response_received_at_before: None,
    ///     response_received_at_after: None,
    ///     cursor: None,
    ///     limit: Some(20),
    /// };
    /// let webhooks = client.list_webhooks(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn list_webhooks(&self, params: &ListWebhooksParams) -> Result<Vec<Webhook>> {
        let path = "/webhooks";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    /// Get a webhook by its id
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - The unique identifier of the webhook
    ///
    /// # Returns
    ///
    /// Returns a [`Webhook`] containing the webhook information.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `401` - Invalid authorization
    /// - `403` - Forbidden
    /// - `404` - Transaction not found
    /// - `500` - Internal server error
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
    /// let webhook_id = Uuid::new_v4();
    /// let webhook = client.get_webhook(&webhook_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_webhook(&self, webhook_id: &Uuid) -> Result<Webhook> {
        let path = format!("/webhooks/{webhook_id}");
        self.get(&path).await
    }

    /// Get all webhooks (blocking)
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters to filter webhooks
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Webhook>`] containing the list of webhooks.
    #[cfg(feature = "sync")]
    pub fn list_webhooks_blocking(&self, params: &ListWebhooksParams) -> Result<Vec<Webhook>> {
        let path = "/webhooks";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }

    /// Get a webhook by its id (blocking)
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - The unique identifier of the webhook
    ///
    /// # Returns
    ///
    /// Returns a [`Webhook`] containing the webhook information.
    #[cfg(feature = "sync")]
    pub fn get_webhook_blocking(&self, webhook_id: &Uuid) -> Result<Webhook> {
        let path = format!("/webhooks/{webhook_id}");
        self.get_blocking(&path)
    }
}
