//! Contracts API
//!
//! This module provides functionality to manage smart contracts.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::contracts::*;
use uuid::Uuid;

impl RainClient {
    /// Get smart contract information for a company
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Contract>`] containing the list of contracts.
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
    /// let contracts = client.get_company_contracts(&company_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_company_contracts(&self, company_id: &Uuid) -> Result<Vec<Contract>> {
        let path = format!("/companies/{company_id}/contracts");
        self.get(&path).await
    }

    /// Create a smart contract for a company
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `request` - The contract creation request
    ///
    /// # Returns
    ///
    /// Returns success (202 Accepted) with no response body.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `404` - Company not found
    /// - `409` - Company already has a contract on this chain
    /// - `500` - Internal server error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::contracts::CreateCompanyContractRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let request = CreateCompanyContractRequest {
    ///     chain_id: 1, // Ethereum mainnet
    ///     owner_address: "0x1234...".to_string(),
    /// };
    /// client.create_company_contract(&company_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_company_contract(
        &self,
        company_id: &Uuid,
        request: &CreateCompanyContractRequest,
    ) -> Result<()> {
        let path = format!("/companies/{company_id}/contracts");
        // Returns 202 Accepted with no body - handle gracefully
        let _: serde_json::Value = self.post(&path, request).await?;
        Ok(())
    }

    /// Get smart contract information for an authorized user tenant
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Contract>`] containing the list of contracts.
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
    /// let contracts = client.get_contracts().await?;
    /// println!("Found {} contracts", contracts.len());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_contracts(&self) -> Result<Vec<Contract>> {
        let path = "/contracts";
        self.get(path).await
    }

    /// Update a smart contract
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The unique identifier of the contract
    /// * `request` - The contract update request
    ///
    /// # Returns
    ///
    /// Returns success (200 OK) with response body.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `404` - Contract not found
    /// - `500` - Internal server error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::contracts::UpdateContractRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let contract_id = Uuid::new_v4();
    /// let request = UpdateContractRequest {
    ///     onramp: true,
    /// };
    /// let response: serde_json::Value = client.update_contract(&contract_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn update_contract(
        &self,
        contract_id: &Uuid,
        request: &UpdateContractRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/contracts/{contract_id}");
        self.put(&path, request).await
    }

    /// Get smart contract information for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Contract>`] containing the list of contracts.
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
    /// let contracts = client.get_user_contracts(&user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_user_contracts(&self, user_id: &Uuid) -> Result<Vec<Contract>> {
        let path = format!("/users/{user_id}/contracts");
        self.get(&path).await
    }

    /// Create a smart contract for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user (must have EVM or Solana address)
    /// * `request` - The contract creation request
    ///
    /// # Returns
    ///
    /// Returns success (202 Accepted) with no response body.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `404` - User not found
    /// - `409` - User already has a contract on this chain
    /// - `500` - Internal server error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::contracts::CreateUserContractRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let user_id = Uuid::new_v4();
    /// let request = CreateUserContractRequest {
    ///     chain_id: 1, // Ethereum mainnet
    /// };
    /// client.create_user_contract(&user_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_user_contract(
        &self,
        user_id: &Uuid,
        request: &CreateUserContractRequest,
    ) -> Result<()> {
        let path = format!("/users/{user_id}/contracts");
        // Returns 202 Accepted with no body - handle gracefully
        let _: serde_json::Value = self.post(&path, request).await?;
        Ok(())
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get smart contract information for a company (blocking)
    #[cfg(feature = "sync")]
    pub fn get_company_contracts_blocking(&self, company_id: &Uuid) -> Result<Vec<Contract>> {
        let path = format!("/companies/{company_id}/contracts");
        self.get_blocking(&path)
    }

    /// Create a smart contract for a company (blocking)
    #[cfg(feature = "sync")]
    pub fn create_company_contract_blocking(
        &self,
        company_id: &Uuid,
        request: &CreateCompanyContractRequest,
    ) -> Result<()> {
        let path = format!("/companies/{company_id}/contracts");
        // Returns 202 Accepted with no body - handle gracefully
        let _: serde_json::Value = self.post_blocking(&path, request)?;
        Ok(())
    }

    /// Get smart contract information for an authorized user tenant (blocking)
    #[cfg(feature = "sync")]
    pub fn get_contracts_blocking(&self) -> Result<Vec<Contract>> {
        let path = "/contracts";
        self.get_blocking(path)
    }

    /// Update a smart contract (blocking)
    #[cfg(feature = "sync")]
    pub fn update_contract_blocking(
        &self,
        contract_id: &Uuid,
        request: &UpdateContractRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/contracts/{contract_id}");
        self.put_blocking(&path, request)
    }

    /// Get smart contract information for a user (blocking)
    #[cfg(feature = "sync")]
    pub fn get_user_contracts_blocking(&self, user_id: &Uuid) -> Result<Vec<Contract>> {
        let path = format!("/users/{user_id}/contracts");
        self.get_blocking(&path)
    }

    /// Create a smart contract for a user (blocking)
    #[cfg(feature = "sync")]
    pub fn create_user_contract_blocking(
        &self,
        user_id: &Uuid,
        request: &CreateUserContractRequest,
    ) -> Result<()> {
        let path = format!("/users/{user_id}/contracts");
        // Returns 202 Accepted with no body - handle gracefully
        let _: serde_json::Value = self.post_blocking(&path, request)?;
        Ok(())
    }
}
