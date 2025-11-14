//! Users API
//!
//! This module provides functionality to manage users.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::charges::*;
use crate::models::users::*;
use uuid::Uuid;

impl RainClient {
    /// Get all users
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering users
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<User>`] containing the list of users.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::users::ListUsersParams;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let params = ListUsersParams {
    ///     company_id: None,
    ///     cursor: None,
    ///     limit: Some(20),
    /// };
    /// let users = client.list_users(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn list_users(&self, params: &ListUsersParams) -> Result<Vec<User>> {
        let mut path = "/issuing/users".to_string();
        let mut query_parts = Vec::new();

        if let Some(ref company_id) = params.company_id {
            query_parts.push(format!("companyId={company_id}"));
        }
        if let Some(ref cursor) = params.cursor {
            query_parts.push(format!("cursor={cursor}"));
        }
        if let Some(limit) = params.limit {
            query_parts.push(format!("limit={limit}"));
        }

        if !query_parts.is_empty() {
            path.push('?');
            path.push_str(&query_parts.join("&"));
        }

        self.get(&path).await
    }

    /// Create an authorized user
    ///
    /// # Arguments
    ///
    /// * `request` - The user creation request
    ///
    /// # Returns
    ///
    /// Returns a [`User`] containing the created user information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::users::CreateUserRequest;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let request = CreateUserRequest {
    ///     first_name: "John".to_string(),
    ///     last_name: "Doe".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     wallet_address: None,
    ///     solana_address: None,
    ///     address: None,
    ///     phone_country_code: None,
    ///     phone_number: None,
    /// };
    /// let user = client.create_user(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_user(&self, request: &CreateUserRequest) -> Result<User> {
        let path = "/issuing/users";
        self.post(path, request).await
    }

    /// Get a user by its ID
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    ///
    /// # Returns
    ///
    /// Returns a [`User`] containing the user information.
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
    /// let user_id = Uuid::new_v4();
    /// let user = client.get_user(&user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_user(&self, user_id: &Uuid) -> Result<User> {
        let path = format!("/issuing/users/{user_id}");
        self.get(&path).await
    }

    /// Delete a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    ///
    /// # Returns
    ///
    /// Returns success (204 No Content).
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
    /// let user_id = Uuid::new_v4();
    /// client.delete_user(&user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn delete_user(&self, user_id: &Uuid) -> Result<()> {
        let path = format!("/issuing/users/{user_id}");
        self.delete(&path).await
    }

    /// Update a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    /// * `request` - The update request
    ///
    /// # Returns
    ///
    /// Returns a [`User`] containing the updated user information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::users::UpdateUserRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let user_id = Uuid::new_v4();
    /// let request = UpdateUserRequest {
    ///     first_name: Some("Jane".to_string()),
    ///     last_name: None,
    ///     email: None,
    ///     is_active: None,
    ///     is_terms_of_service_accepted: None,
    ///     address: None,
    ///     phone_country_code: None,
    ///     phone_number: None,
    ///     wallet_address: None,
    ///     solana_address: None,
    ///     tron_address: None,
    ///     stellar_address: None,
    /// };
    /// let user = client.update_user(&user_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn update_user(&self, user_id: &Uuid, request: &UpdateUserRequest) -> Result<User> {
        let path = format!("/issuing/users/{user_id}");
        self.patch(&path, request).await
    }

    /// Charge a user a custom fee
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    /// * `request` - The charge request
    ///
    /// # Returns
    ///
    /// Returns a [`Charge`] containing the created charge information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::charges::CreateChargeRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let user_id = Uuid::new_v4();
    /// let request = CreateChargeRequest {
    ///     amount: 500, // $5.00 in cents
    ///     description: "Custom fee".to_string(),
    /// };
    /// let charge = client.charge_user(&user_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn charge_user(
        &self,
        user_id: &Uuid,
        request: &CreateChargeRequest,
    ) -> Result<Charge> {
        let path = format!("/issuing/users/{user_id}/charges");
        self.post(&path, request).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get all users (blocking)
    #[cfg(feature = "sync")]
    pub fn list_users_blocking(&self, params: &ListUsersParams) -> Result<Vec<User>> {
        let mut path = "/issuing/users".to_string();
        let mut query_parts = Vec::new();

        if let Some(ref company_id) = params.company_id {
            query_parts.push(format!("companyId={company_id}"));
        }
        if let Some(ref cursor) = params.cursor {
            query_parts.push(format!("cursor={cursor}"));
        }
        if let Some(limit) = params.limit {
            query_parts.push(format!("limit={limit}"));
        }

        if !query_parts.is_empty() {
            path.push('?');
            path.push_str(&query_parts.join("&"));
        }

        self.get_blocking(&path)
    }

    /// Create an authorized user (blocking)
    #[cfg(feature = "sync")]
    pub fn create_user_blocking(&self, request: &CreateUserRequest) -> Result<User> {
        let path = "/issuing/users";
        self.post_blocking(path, request)
    }

    /// Get a user by its ID (blocking)
    #[cfg(feature = "sync")]
    pub fn get_user_blocking(&self, user_id: &Uuid) -> Result<User> {
        let path = format!("/issuing/users/{user_id}");
        self.get_blocking(&path)
    }

    /// Delete a user (blocking)
    #[cfg(feature = "sync")]
    pub fn delete_user_blocking(&self, user_id: &Uuid) -> Result<()> {
        let path = format!("/issuing/users/{user_id}");
        self.delete_blocking(&path)
    }

    /// Update a user (blocking)
    #[cfg(feature = "sync")]
    pub fn update_user_blocking(
        &self,
        user_id: &Uuid,
        request: &UpdateUserRequest,
    ) -> Result<User> {
        let path = format!("/issuing/users/{user_id}");
        self.patch_blocking(&path, request)
    }

    /// Charge a user a custom fee (blocking)
    #[cfg(feature = "sync")]
    pub fn charge_user_blocking(
        &self,
        user_id: &Uuid,
        request: &CreateChargeRequest,
    ) -> Result<Charge> {
        let path = format!("/issuing/users/{user_id}/charges");
        self.post_blocking(&path, request)
    }
}
