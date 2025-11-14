//! Balances API
//!
//! This module provides functionality to retrieve balances for tenants, companies, and users.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::balances::BalanceResponse;
use uuid::Uuid;

impl RainClient {
    /// Get a tenant's credit balances
    ///
    /// # Returns
    ///
    /// Returns a [`BalanceResponse`] containing the tenant's balance information.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `401` - Invalid authorization
    /// - `500` - Internal server error
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
    /// let balance = client.get_balances().await?;
    /// println!("Credit limit: {}", balance.credit_limit);
    /// println!("Spending power: {}", balance.spending_power);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_balances(&self) -> Result<BalanceResponse> {
        let path = "/balances";
        self.get(path).await
    }

    /// Get a company's credit balances
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    ///
    /// # Returns
    ///
    /// Returns a [`BalanceResponse`] containing the company's balance information.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `401` - Invalid authorization
    /// - `404` - Company not found
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
    /// let company_id = Uuid::new_v4();
    /// let balance = client.get_company_balances(&company_id).await?;
    /// println!("Credit limit: {}", balance.credit_limit);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_company_balances(&self, company_id: &Uuid) -> Result<BalanceResponse> {
        let path = format!("/companies/{company_id}/balances");
        self.get(&path).await
    }

    /// Get a user's credit balances
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    ///
    /// # Returns
    ///
    /// Returns a [`BalanceResponse`] containing the user's balance information.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `401` - Invalid authorization
    /// - `404` - User not found
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
    /// let user_id = Uuid::new_v4();
    /// let balance = client.get_user_balances(&user_id).await?;
    /// println!("Credit limit: {}", balance.credit_limit);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_user_balances(&self, user_id: &Uuid) -> Result<BalanceResponse> {
        let path = format!("/users/{user_id}/balances");
        self.get(&path).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get a tenant's credit balances (blocking)
    #[cfg(feature = "sync")]
    pub fn get_balances_blocking(&self) -> Result<BalanceResponse> {
        let path = "/balances";
        self.get_blocking(path)
    }

    /// Get a company's credit balances (blocking)
    #[cfg(feature = "sync")]
    pub fn get_company_balances_blocking(&self, company_id: &Uuid) -> Result<BalanceResponse> {
        let path = format!("/companies/{company_id}/balances");
        self.get_blocking(&path)
    }

    /// Get a user's credit balances (blocking)
    #[cfg(feature = "sync")]
    pub fn get_user_balances_blocking(&self, user_id: &Uuid) -> Result<BalanceResponse> {
        let path = format!("/users/{user_id}/balances");
        self.get_blocking(&path)
    }
}
