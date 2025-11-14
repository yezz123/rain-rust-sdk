//! Payments API
//!
//! This module provides functionality to initiate payments.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::payments::*;
use uuid::Uuid;

impl RainClient {
    /// Initiate a payment for a company
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `request` - The payment initiation request
    ///
    /// # Returns
    ///
    /// Returns an [`InitiatePaymentResponse`] containing the payment address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::payments::InitiatePaymentRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let request = InitiatePaymentRequest {
    ///     amount: 10000, // $100.00 in cents
    ///     wallet_address: "0x1234...".to_string(),
    ///     chain_id: Some(1), // Ethereum mainnet
    /// };
    /// let response = client.initiate_company_payment(&company_id, &request).await?;
    /// println!("Payment address: {}", response.address);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn initiate_company_payment(
        &self,
        company_id: &Uuid,
        request: &InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse> {
        let path = format!("/issuing/companies/{company_id}/payments");
        self.post(&path, request).await
    }

    /// Initiate a payment for an authorized user tenant
    ///
    /// # Arguments
    ///
    /// * `request` - The payment initiation request
    ///
    /// # Returns
    ///
    /// Returns an [`InitiatePaymentResponse`] containing the payment address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::payments::InitiatePaymentRequest;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let request = InitiatePaymentRequest {
    ///     amount: 5000, // $50.00 in cents
    ///     wallet_address: "0x5678...".to_string(),
    ///     chain_id: Some(137), // Polygon
    /// };
    /// let response = client.initiate_payment(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn initiate_payment(
        &self,
        request: &InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse> {
        let path = "/issuing/payments";
        self.post(path, request).await
    }

    /// Initiate a payment for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    /// * `request` - The payment initiation request
    ///
    /// # Returns
    ///
    /// Returns an [`InitiatePaymentResponse`] containing the payment address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::payments::InitiatePaymentRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let user_id = Uuid::new_v4();
    /// let request = InitiatePaymentRequest {
    ///     amount: 2500, // $25.00 in cents
    ///     wallet_address: "0xabcd...".to_string(),
    ///     chain_id: Some(1),
    /// };
    /// let response = client.initiate_user_payment(&user_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn initiate_user_payment(
        &self,
        user_id: &Uuid,
        request: &InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse> {
        let path = format!("/issuing/users/{user_id}/payments");
        self.post(&path, request).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Initiate a payment for a company (blocking)
    #[cfg(feature = "sync")]
    pub fn initiate_company_payment_blocking(
        &self,
        company_id: &Uuid,
        request: &InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse> {
        let path = format!("/issuing/companies/{company_id}/payments");
        self.post_blocking(&path, request)
    }

    /// Initiate a payment for an authorized user tenant (blocking)
    #[cfg(feature = "sync")]
    pub fn initiate_payment_blocking(
        &self,
        request: &InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse> {
        let path = "/issuing/payments";
        self.post_blocking(path, request)
    }

    /// Initiate a payment for a user (blocking)
    #[cfg(feature = "sync")]
    pub fn initiate_user_payment_blocking(
        &self,
        user_id: &Uuid,
        request: &InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse> {
        let path = format!("/issuing/users/{user_id}/payments");
        self.post_blocking(&path, request)
    }
}
