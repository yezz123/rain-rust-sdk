//! Subtenants API
//!
//! This module provides functionality to manage subtenants.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::subtenants::*;
use uuid::Uuid;

impl RainClient {
    /// Get all subtenants
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Subtenant>`] containing the list of subtenants.
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
    /// let subtenants = client.list_subtenants().await?;
    /// println!("Found {} subtenants", subtenants.len());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn list_subtenants(&self) -> Result<Vec<Subtenant>> {
        let path = "/issuing/subtenants";
        self.get(path).await
    }

    /// Create a subtenant
    ///
    /// # Arguments
    ///
    /// * `request` - The subtenant creation request
    ///
    /// # Returns
    ///
    /// Returns a [`Subtenant`] containing the created subtenant information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::subtenants::CreateSubtenantRequest;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let request = CreateSubtenantRequest {
    ///     name: Some("My Subtenant".to_string()),
    /// };
    /// let subtenant = client.create_subtenant(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_subtenant(&self, request: &CreateSubtenantRequest) -> Result<Subtenant> {
        let path = "/issuing/subtenants";
        self.post(path, request).await
    }

    /// Get a subtenant by its id
    ///
    /// # Arguments
    ///
    /// * `subtenant_id` - The unique identifier of the subtenant
    ///
    /// # Returns
    ///
    /// Returns a [`Subtenant`] containing the subtenant information.
    #[cfg(feature = "async")]
    pub async fn get_subtenant(&self, subtenant_id: &Uuid) -> Result<Subtenant> {
        let path = format!("/issuing/subtenants/{subtenant_id}");
        self.get(&path).await
    }

    /// Update a subtenant
    ///
    /// # Arguments
    ///
    /// * `subtenant_id` - The unique identifier of the subtenant
    /// * `request` - The update request
    ///
    /// # Returns
    ///
    /// Returns success (204 No Content) with no response body.
    #[cfg(feature = "async")]
    pub async fn update_subtenant(
        &self,
        subtenant_id: &Uuid,
        request: &UpdateSubtenantRequest,
    ) -> Result<()> {
        let path = format!("/issuing/subtenants/{subtenant_id}");
        let _: serde_json::Value = self.patch(&path, request).await?;
        Ok(())
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get all subtenants (blocking)
    #[cfg(feature = "sync")]
    pub fn list_subtenants_blocking(&self) -> Result<Vec<Subtenant>> {
        let path = "/issuing/subtenants";
        self.get_blocking(path)
    }

    /// Create a subtenant (blocking)
    #[cfg(feature = "sync")]
    pub fn create_subtenant_blocking(&self, request: &CreateSubtenantRequest) -> Result<Subtenant> {
        let path = "/issuing/subtenants";
        self.post_blocking(path, request)
    }

    /// Get a subtenant by its id (blocking)
    #[cfg(feature = "sync")]
    pub fn get_subtenant_blocking(&self, subtenant_id: &Uuid) -> Result<Subtenant> {
        let path = format!("/issuing/subtenants/{subtenant_id}");
        self.get_blocking(&path)
    }

    /// Update a subtenant (blocking)
    #[cfg(feature = "sync")]
    pub fn update_subtenant_blocking(
        &self,
        subtenant_id: &Uuid,
        request: &UpdateSubtenantRequest,
    ) -> Result<()> {
        let path = format!("/issuing/subtenants/{subtenant_id}");
        let _: serde_json::Value = self.patch_blocking(&path, request)?;
        Ok(())
    }
}
