//! Shipping Groups API
//!
//! This module provides functionality to manage bulk shipping groups.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::shipping_groups::*;
use uuid::Uuid;

impl RainClient {
    /// Get all bulk shipping groups
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering shipping groups
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<ShippingGroup>`] containing the list of shipping groups.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::shipping_groups::{ListShippingGroupsParams, ShippingGroup};
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let params = ListShippingGroupsParams {
    ///     cursor: None,
    ///     limit: Some(20),
    /// };
    /// let shipping_groups = client.list_shipping_groups(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn list_shipping_groups(
        &self,
        params: &ListShippingGroupsParams,
    ) -> Result<Vec<ShippingGroup>> {
        let path = "/issuing/shipping-groups";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    /// Create a bulk shipping group
    ///
    /// # Arguments
    ///
    /// * `request` - The shipping group creation request
    ///
    /// # Returns
    ///
    /// Returns a [`ShippingGroup`] containing the created shipping group information (202 Accepted).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::shipping_groups::CreateShippingGroupRequest;
    /// use rain_sdk::models::common::Address;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let request = CreateShippingGroupRequest {
    ///     recipient_first_name: "John".to_string(),
    ///     recipient_last_name: Some("Doe".to_string()),
    ///     recipient_phone_country_code: Some("1".to_string()),
    ///     recipient_phone_number: Some("5555555555".to_string()),
    ///     address: Address {
    ///         line1: "123 Main St".to_string(),
    ///         line2: None,
    ///         city: "New York".to_string(),
    ///         region: "NY".to_string(),
    ///         postal_code: "10001".to_string(),
    ///         country_code: "US".to_string(),
    ///         country: None,
    ///     },
    /// };
    /// let shipping_group = client.create_shipping_group(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_shipping_group(
        &self,
        request: &CreateShippingGroupRequest,
    ) -> Result<ShippingGroup> {
        let path = "/issuing/shipping-groups";
        // Returns 202 Accepted
        self.post(path, request).await
    }

    /// Get a bulk shipping group by its id
    ///
    /// # Arguments
    ///
    /// * `shipping_group_id` - The unique identifier of the shipping group
    ///
    /// # Returns
    ///
    /// Returns a [`ShippingGroup`] containing the shipping group information.
    #[cfg(feature = "async")]
    pub async fn get_shipping_group(&self, shipping_group_id: &Uuid) -> Result<ShippingGroup> {
        let path = format!("/issuing/shipping-groups/{shipping_group_id}");
        self.get(&path).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get all bulk shipping groups (blocking)
    #[cfg(feature = "sync")]
    pub fn list_shipping_groups_blocking(
        &self,
        params: &ListShippingGroupsParams,
    ) -> Result<Vec<ShippingGroup>> {
        let path = "/issuing/shipping-groups";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }

    /// Create a bulk shipping group (blocking)
    #[cfg(feature = "sync")]
    pub fn create_shipping_group_blocking(
        &self,
        request: &CreateShippingGroupRequest,
    ) -> Result<ShippingGroup> {
        let path = "/issuing/shipping-groups";
        // Returns 202 Accepted
        self.post_blocking(path, request)
    }

    /// Get a bulk shipping group by its id (blocking)
    #[cfg(feature = "sync")]
    pub fn get_shipping_group_blocking(&self, shipping_group_id: &Uuid) -> Result<ShippingGroup> {
        let path = format!("/issuing/shipping-groups/{shipping_group_id}");
        self.get_blocking(&path)
    }
}
