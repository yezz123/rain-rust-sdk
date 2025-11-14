//! Companies API
//!
//! This module provides functionality to manage companies.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::charges::*;
use crate::models::companies::*;
use uuid::Uuid;

impl RainClient {
    /// Get all companies
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering companies
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Company>`] containing the list of companies.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::companies::ListCompaniesParams;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let params = ListCompaniesParams {
    ///     cursor: None,
    ///     limit: Some(20),
    /// };
    /// let companies = client.list_companies(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn list_companies(&self, params: &ListCompaniesParams) -> Result<Vec<Company>> {
        let mut path = "/issuing/companies".to_string();
        let mut query_parts = Vec::new();

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

    /// Get a company by its ID
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    ///
    /// # Returns
    ///
    /// Returns a [`Company`] containing the company information.
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
    /// let company = client.get_company(&company_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_company(&self, company_id: &Uuid) -> Result<Company> {
        let path = format!("/issuing/companies/{company_id}");
        self.get(&path).await
    }

    /// Update a company
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `request` - The update request
    ///
    /// # Returns
    ///
    /// Returns a [`Company`] containing the updated company information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::companies::UpdateCompanyRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let request = UpdateCompanyRequest {
    ///     name: Some("New Company Name".to_string()),
    ///     address: None,
    /// };
    /// let company = client.update_company(&company_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn update_company(
        &self,
        company_id: &Uuid,
        request: &UpdateCompanyRequest,
    ) -> Result<Company> {
        let path = format!("/issuing/companies/{company_id}");
        self.patch(&path, request).await
    }

    /// Charge a company a custom fee
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
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
    /// let company_id = Uuid::new_v4();
    /// let request = CreateChargeRequest {
    ///     amount: 1000, // $10.00 in cents
    ///     description: "Custom fee".to_string(),
    /// };
    /// let charge = client.charge_company(&company_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn charge_company(
        &self,
        company_id: &Uuid,
        request: &CreateChargeRequest,
    ) -> Result<Charge> {
        let path = format!("/issuing/companies/{company_id}/charges");
        self.post(&path, request).await
    }

    /// Create a user in a company
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `request` - The user creation request
    ///
    /// # Returns
    ///
    /// Returns a [`crate::models::User`] containing the created user information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::users::CreateCompanyUserRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let request = CreateCompanyUserRequest {
    ///     first_name: "John".to_string(),
    ///     last_name: "Doe".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     is_terms_of_service_accepted: true,
    ///     birth_date: None,
    ///     wallet_address: None,
    ///     solana_address: None,
    ///     address: None,
    ///     phone_country_code: None,
    ///     phone_number: None,
    /// };
    /// let user = client.create_company_user(&company_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_company_user(
        &self,
        company_id: &Uuid,
        request: &crate::models::users::CreateCompanyUserRequest,
    ) -> Result<crate::models::users::User> {
        let path = format!("/issuing/companies/{company_id}/users");
        self.post(&path, request).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get all companies (blocking)
    #[cfg(feature = "sync")]
    pub fn list_companies_blocking(&self, params: &ListCompaniesParams) -> Result<Vec<Company>> {
        let mut path = "/issuing/companies".to_string();
        let mut query_parts = Vec::new();

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

    /// Get a company by its ID (blocking)
    #[cfg(feature = "sync")]
    pub fn get_company_blocking(&self, company_id: &Uuid) -> Result<Company> {
        let path = format!("/issuing/companies/{company_id}");
        self.get_blocking(&path)
    }

    /// Update a company (blocking)
    #[cfg(feature = "sync")]
    pub fn update_company_blocking(
        &self,
        company_id: &Uuid,
        request: &UpdateCompanyRequest,
    ) -> Result<Company> {
        let path = format!("/issuing/companies/{company_id}");
        self.patch_blocking(&path, request)
    }

    /// Charge a company a custom fee (blocking)
    #[cfg(feature = "sync")]
    pub fn charge_company_blocking(
        &self,
        company_id: &Uuid,
        request: &CreateChargeRequest,
    ) -> Result<Charge> {
        let path = format!("/issuing/companies/{company_id}/charges");
        self.post_blocking(&path, request)
    }

    /// Create a user in a company (blocking)
    #[cfg(feature = "sync")]
    pub fn create_company_user_blocking(
        &self,
        company_id: &Uuid,
        request: &crate::models::users::CreateCompanyUserRequest,
    ) -> Result<crate::models::users::User> {
        let path = format!("/issuing/companies/{company_id}/users");
        self.post_blocking(&path, request)
    }
}
