//! Signatures API
//!
//! This module provides functionality to get payment and withdrawal signatures.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::signatures::*;
use uuid::Uuid;

impl RainClient {
    /// Get payment signature for a company
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `params` - Query parameters for the signature request
    ///
    /// # Returns
    ///
    /// Returns a [`PaymentSignatureResponse`] which can be either pending or ready.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `404` - Company not found
    /// - `409` - Another active signature already exists
    /// - `500` - Internal server error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::signatures::PaymentSignatureParams;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let params = PaymentSignatureParams {
    ///     chain_id: Some(1),
    ///     token: "0xabc123...".to_string(),
    ///     amount: "1000000".to_string(),
    ///     admin_address: "0xdef456...".to_string(),
    ///     is_amount_native: Some(false),
    ///     rain_collateral_contract_id: None,
    /// };
    /// let response = client.get_company_payment_signature(&company_id, &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_company_payment_signature(
        &self,
        company_id: &Uuid,
        params: &PaymentSignatureParams,
    ) -> Result<PaymentSignatureResponse> {
        let path = format!("/companies/{company_id}/signatures/payments");
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    /// Get withdrawal signature for a company
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `params` - Query parameters for the signature request
    ///
    /// # Returns
    ///
    /// Returns a [`WithdrawalSignatureResponse`] which can be either pending or ready.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `404` - Company not found
    /// - `409` - Another active signature already exists
    /// - `500` - Internal server error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::signatures::WithdrawalSignatureParams;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let params = WithdrawalSignatureParams {
    ///     chain_id: Some(1),
    ///     token: "0xabc123...".to_string(),
    ///     amount: "500000".to_string(),
    ///     admin_address: "0xdef456...".to_string(),
    ///     recipient_address: "0x789ghi...".to_string(),
    ///     is_amount_native: Some(false),
    ///     rain_collateral_contract_id: None,
    /// };
    /// let response = client.get_company_withdrawal_signature(&company_id, &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_company_withdrawal_signature(
        &self,
        company_id: &Uuid,
        params: &WithdrawalSignatureParams,
    ) -> Result<WithdrawalSignatureResponse> {
        let path = format!("/companies/{company_id}/signatures/withdrawals");
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    /// Get payment signature for an authorized user tenant
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for the signature request
    ///
    /// # Returns
    ///
    /// Returns a [`PaymentSignatureResponse`] which can be either pending or ready.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `409` - Another active signature already exists
    /// - `500` - Internal server error
    #[cfg(feature = "async")]
    pub async fn get_payment_signature(
        &self,
        params: &PaymentSignatureParams,
    ) -> Result<PaymentSignatureResponse> {
        let path = "/signatures/payments";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    /// Get withdrawal signature for an authorized user tenant
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for the signature request
    ///
    /// # Returns
    ///
    /// Returns a [`WithdrawalSignatureResponse`] which can be either pending or ready.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `409` - Another active signature already exists
    /// - `500` - Internal server error
    #[cfg(feature = "async")]
    pub async fn get_withdrawal_signature(
        &self,
        params: &WithdrawalSignatureParams,
    ) -> Result<WithdrawalSignatureResponse> {
        let path = "/signatures/withdrawals";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    /// Get payment signature for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    /// * `params` - Query parameters for the signature request
    ///
    /// # Returns
    ///
    /// Returns a [`PaymentSignatureResponse`] which can be either pending or ready.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `404` - User not found
    /// - `409` - Another active signature already exists
    /// - `500` - Internal server error
    #[cfg(feature = "async")]
    pub async fn get_user_payment_signature(
        &self,
        user_id: &Uuid,
        params: &PaymentSignatureParams,
    ) -> Result<PaymentSignatureResponse> {
        let path = format!("/users/{user_id}/signatures/payments");
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    /// Get withdrawal signature for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    /// * `params` - Query parameters for the signature request
    ///
    /// # Returns
    ///
    /// Returns a [`WithdrawalSignatureResponse`] which can be either pending or ready.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `404` - User not found
    /// - `409` - Another active signature already exists
    /// - `500` - Internal server error
    #[cfg(feature = "async")]
    pub async fn get_user_withdrawal_signature(
        &self,
        user_id: &Uuid,
        params: &WithdrawalSignatureParams,
    ) -> Result<WithdrawalSignatureResponse> {
        let path = format!("/users/{user_id}/signatures/withdrawals");
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get payment signature for a company (blocking)
    #[cfg(feature = "sync")]
    pub fn get_company_payment_signature_blocking(
        &self,
        company_id: &Uuid,
        params: &PaymentSignatureParams,
    ) -> Result<PaymentSignatureResponse> {
        let path = format!("/companies/{company_id}/signatures/payments");
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }

    /// Get withdrawal signature for a company (blocking)
    #[cfg(feature = "sync")]
    pub fn get_company_withdrawal_signature_blocking(
        &self,
        company_id: &Uuid,
        params: &WithdrawalSignatureParams,
    ) -> Result<WithdrawalSignatureResponse> {
        let path = format!("/companies/{company_id}/signatures/withdrawals");
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }

    /// Get payment signature for an authorized user tenant (blocking)
    #[cfg(feature = "sync")]
    pub fn get_payment_signature_blocking(
        &self,
        params: &PaymentSignatureParams,
    ) -> Result<PaymentSignatureResponse> {
        let path = "/signatures/payments";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }

    /// Get withdrawal signature for an authorized user tenant (blocking)
    #[cfg(feature = "sync")]
    pub fn get_withdrawal_signature_blocking(
        &self,
        params: &WithdrawalSignatureParams,
    ) -> Result<WithdrawalSignatureResponse> {
        let path = "/signatures/withdrawals";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }

    /// Get payment signature for a user (blocking)
    #[cfg(feature = "sync")]
    pub fn get_user_payment_signature_blocking(
        &self,
        user_id: &Uuid,
        params: &PaymentSignatureParams,
    ) -> Result<PaymentSignatureResponse> {
        let path = format!("/users/{user_id}/signatures/payments");
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }

    /// Get withdrawal signature for a user (blocking)
    #[cfg(feature = "sync")]
    pub fn get_user_withdrawal_signature_blocking(
        &self,
        user_id: &Uuid,
        params: &WithdrawalSignatureParams,
    ) -> Result<WithdrawalSignatureResponse> {
        let path = format!("/users/{user_id}/signatures/withdrawals");
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }
}
