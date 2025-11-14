//! Applications API
//!
//! This module provides functionality to manage company and user applications in the Rain API.
//!
//! # Examples
//!
//! ```no_run
//! use rain_sdk::{RainClient, Config, Environment, AuthConfig};
//! use rain_sdk::models::applications::*;
//! use uuid::Uuid;
//!
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::new(Environment::Dev);
//! let auth = AuthConfig::with_api_key("your-api-key".to_string());
//! let client = RainClient::new(config, auth)?;
//!
//! // Create a company application
//! let company_id = Uuid::new_v4();
//! let application = client.create_company_application(&request).await?;
//! println!("Application: {:?}", application);
//! # Ok(())
//! # }
//! ```

use crate::client::RainClient;
use crate::error::Result;
use crate::models::applications::*;
use uuid::Uuid;

impl RainClient {
    // ============================================================================
    // Company Application Methods
    // ============================================================================

    /// Create a company application
    ///
    /// # Arguments
    ///
    /// * `request` - The company application request
    ///
    /// # Returns
    ///
    /// Returns a [`CompanyApplicationResponse`] containing the application information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::applications::CreateCompanyApplicationRequest;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let request = CreateCompanyApplicationRequest { /* ... */ };
    /// let application = client.create_company_application(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_company_application(
        &self,
        request: &CreateCompanyApplicationRequest,
    ) -> Result<CompanyApplicationResponse> {
        let path = "/issuing/applications/company";
        self.post(path, request).await
    }

    /// Get a company application by ID
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    ///
    /// # Returns
    ///
    /// Returns a [`CompanyApplicationResponse`] containing the application information.
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
    /// let application = client.get_company_application(&company_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_company_application(
        &self,
        company_id: &Uuid,
    ) -> Result<CompanyApplicationResponse> {
        let path = format!("/issuing/applications/company/{company_id}");
        self.get(&path).await
    }

    /// Update a company application
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `request` - The update request
    ///
    /// # Returns
    ///
    /// Returns a [`CompanyApplicationResponse`] containing the updated application information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::applications::UpdateCompanyApplicationRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let request = UpdateCompanyApplicationRequest { /* ... */ };
    /// let application = client.update_company_application(&company_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn update_company_application(
        &self,
        company_id: &Uuid,
        request: &UpdateCompanyApplicationRequest,
    ) -> Result<CompanyApplicationResponse> {
        let path = format!("/issuing/applications/company/{company_id}");
        self.patch(&path, request).await
    }

    /// Update an ultimate beneficial owner
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `ubo_id` - The unique identifier of the ultimate beneficial owner
    /// * `request` - The update request
    ///
    /// # Returns
    ///
    /// Returns a [`CompanyApplicationResponse`] containing the updated application information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::applications::UpdateUltimateBeneficialOwnerRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let ubo_id = Uuid::new_v4();
    /// let request = UpdateUltimateBeneficialOwnerRequest { /* ... */ };
    /// let application = client.update_ultimate_beneficial_owner(&company_id, &ubo_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn update_ultimate_beneficial_owner(
        &self,
        company_id: &Uuid,
        ubo_id: &Uuid,
        request: &UpdateUltimateBeneficialOwnerRequest,
    ) -> Result<CompanyApplicationResponse> {
        let path = format!("/issuing/applications/company/{company_id}/ubo/{ubo_id}");
        self.patch(&path, request).await
    }

    /// Upload a document for a company application
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `params` - Document upload parameters
    ///
    /// # Returns
    ///
    /// Returns a success response.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::applications::DocumentUploadParams;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let params = DocumentUploadParams {
    ///     document_type: "directorsRegistry".to_string(),
    ///     side: "front".to_string(),
    ///     country: Some("US".to_string()),
    ///     country_code: Some("US".to_string()),
    ///     name: Some("Document Name".to_string()),
    ///     file_path: "/path/to/file.pdf".to_string(),
    /// };
    /// client.upload_company_document(&company_id, &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn upload_company_document(
        &self,
        company_id: &Uuid,
        params: &DocumentUploadParams,
    ) -> Result<serde_json::Value> {
        let path = format!("/issuing/applications/company/{company_id}/document");
        let form = self.build_company_document_form(params)?;
        self.put_multipart(&path, form).await
    }

    /// Upload a document for an ultimate beneficial owner
    ///
    /// # Arguments
    ///
    /// * `company_id` - The unique identifier of the company
    /// * `ubo_id` - The unique identifier of the ultimate beneficial owner
    /// * `params` - Document upload parameters
    ///
    /// # Returns
    ///
    /// Returns a success response.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::applications::DocumentUploadParams;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let company_id = Uuid::new_v4();
    /// let ubo_id = Uuid::new_v4();
    /// let params = DocumentUploadParams {
    ///     document_type: "idCard".to_string(),
    ///     side: "front".to_string(),
    ///     country: Some("US".to_string()),
    ///     country_code: Some("US".to_string()),
    ///     name: None,
    ///     file_path: "/path/to/file.pdf".to_string(),
    /// };
    /// client.upload_ubo_document(&company_id, &ubo_id, &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn upload_ubo_document(
        &self,
        company_id: &Uuid,
        ubo_id: &Uuid,
        params: &DocumentUploadParams,
    ) -> Result<serde_json::Value> {
        let path = format!("/issuing/applications/company/{company_id}/ubo/{ubo_id}/document");
        let form = self.build_user_document_form(params)?;
        self.put_multipart(&path, form).await
    }

    // ============================================================================
    // User Application Methods
    // ============================================================================

    /// Create a user application
    ///
    /// # Arguments
    ///
    /// * `request` - The user application request
    ///
    /// # Returns
    ///
    /// Returns a [`UserApplicationResponse`] containing the application information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::applications::CreateUserApplicationRequest;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let request = CreateUserApplicationRequest { /* ... */ };
    /// let application = client.create_user_application(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_user_application(
        &self,
        request: &CreateUserApplicationRequest,
    ) -> Result<UserApplicationResponse> {
        let path = "/issuing/applications/user";
        self.post(path, request).await
    }

    /// Initiate a user application
    ///
    /// # Arguments
    ///
    /// * `request` - The initiate user application request
    ///
    /// # Returns
    ///
    /// Returns a [`UserApplicationResponse`] containing the application information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::applications::InitiateUserApplicationRequest;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let request = InitiateUserApplicationRequest {
    ///     first_name: "John".to_string(),
    ///     last_name: "Doe".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     wallet_address: None,
    /// };
    /// let application = client.initiate_user_application(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn initiate_user_application(
        &self,
        request: &InitiateUserApplicationRequest,
    ) -> Result<UserApplicationResponse> {
        let path = "/issuing/applications/user/initiate";
        self.post(path, request).await
    }

    /// Get a user application by ID
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    ///
    /// # Returns
    ///
    /// Returns a [`UserApplicationResponse`] containing the application information.
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
    /// let application = client.get_user_application(&user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_user_application(&self, user_id: &Uuid) -> Result<UserApplicationResponse> {
        let path = format!("/issuing/applications/user/{user_id}");
        self.get(&path).await
    }

    /// Update a user application
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    /// * `request` - The update request
    ///
    /// # Returns
    ///
    /// Returns a [`UserApplicationResponse`] containing the updated application information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::applications::UpdateUserApplicationRequest;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let user_id = Uuid::new_v4();
    /// let request = UpdateUserApplicationRequest { /* ... */ };
    /// let application = client.update_user_application(&user_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn update_user_application(
        &self,
        user_id: &Uuid,
        request: &UpdateUserApplicationRequest,
    ) -> Result<UserApplicationResponse> {
        let path = format!("/issuing/applications/user/{user_id}");
        self.patch(&path, request).await
    }

    /// Upload a document for a user application
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    /// * `params` - Document upload parameters
    ///
    /// # Returns
    ///
    /// Returns a success response.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::applications::DocumentUploadParams;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let user_id = Uuid::new_v4();
    /// let params = DocumentUploadParams {
    ///     document_type: "idCard".to_string(),
    ///     side: "front".to_string(),
    ///     country: Some("US".to_string()),
    ///     country_code: Some("US".to_string()),
    ///     name: Some("ID Card".to_string()),
    ///     file_path: "/path/to/file.pdf".to_string(),
    /// };
    /// client.upload_user_document(&user_id, &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn upload_user_document(
        &self,
        user_id: &Uuid,
        params: &DocumentUploadParams,
    ) -> Result<serde_json::Value> {
        let path = format!("/issuing/applications/user/{user_id}/document");
        let form = self.build_user_document_form(params)?;
        self.put_multipart(&path, form).await
    }

    // ============================================================================
    // Helper Methods
    // ============================================================================

    #[cfg(feature = "async")]
    fn build_company_document_form(
        &self,
        params: &DocumentUploadParams,
    ) -> Result<reqwest::multipart::Form> {
        use std::fs;

        let file_bytes = fs::read(&params.file_path).map_err(|e| {
            crate::error::RainError::Other(anyhow::anyhow!("Failed to read file: {e}"))
        })?;

        let file_name = params
            .file_path
            .split('/')
            .next_back()
            .unwrap_or("document")
            .to_string();

        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .map_err(|e| {
                crate::error::RainError::Other(anyhow::anyhow!("Invalid MIME type: {e}"))
            })?;

        let mut form = reqwest::multipart::Form::new()
            .part("document", part)
            .text("type", params.document_type.clone())
            .text("side", params.side.clone());

        if let Some(ref name) = params.name {
            form = form.text("name", name.clone());
        }

        if let Some(ref country) = params.country {
            form = form.text("country", country.clone());
        }

        if let Some(ref country_code) = params.country_code {
            form = form.text("countryCode", country_code.clone());
        }

        Ok(form)
    }

    #[cfg(feature = "async")]
    fn build_user_document_form(
        &self,
        params: &DocumentUploadParams,
    ) -> Result<reqwest::multipart::Form> {
        use std::fs;

        let file_bytes = fs::read(&params.file_path).map_err(|e| {
            crate::error::RainError::Other(anyhow::anyhow!("Failed to read file: {e}"))
        })?;

        let file_name = params
            .file_path
            .split('/')
            .next_back()
            .unwrap_or("document")
            .to_string();

        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .map_err(|e| {
                crate::error::RainError::Other(anyhow::anyhow!("Invalid MIME type: {e}"))
            })?;

        let mut form = reqwest::multipart::Form::new()
            .part("document", part)
            .text("type", params.document_type.clone())
            .text("side", params.side.clone());

        if let Some(ref name) = params.name {
            form = form.text("name", name.clone());
        }

        if let Some(ref country) = params.country {
            form = form.text("country", country.clone());
        }

        if let Some(ref country_code) = params.country_code {
            form = form.text("countryCode", country_code.clone());
        }

        Ok(form)
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Create a company application (blocking)
    #[cfg(feature = "sync")]
    pub fn create_company_application_blocking(
        &self,
        request: &CreateCompanyApplicationRequest,
    ) -> Result<CompanyApplicationResponse> {
        let path = "/issuing/applications/company";
        self.post_blocking(path, request)
    }

    /// Get a company application by ID (blocking)
    #[cfg(feature = "sync")]
    pub fn get_company_application_blocking(
        &self,
        company_id: &Uuid,
    ) -> Result<CompanyApplicationResponse> {
        let path = format!("/issuing/applications/company/{company_id}");
        self.get_blocking(&path)
    }

    /// Update a company application (blocking)
    #[cfg(feature = "sync")]
    pub fn update_company_application_blocking(
        &self,
        company_id: &Uuid,
        request: &UpdateCompanyApplicationRequest,
    ) -> Result<CompanyApplicationResponse> {
        let path = format!("/issuing/applications/company/{company_id}");
        self.patch_blocking(&path, request)
    }

    /// Update an ultimate beneficial owner (blocking)
    #[cfg(feature = "sync")]
    pub fn update_ultimate_beneficial_owner_blocking(
        &self,
        company_id: &Uuid,
        ubo_id: &Uuid,
        request: &UpdateUltimateBeneficialOwnerRequest,
    ) -> Result<CompanyApplicationResponse> {
        let path = format!("/issuing/applications/company/{company_id}/ubo/{ubo_id}");
        self.patch_blocking(&path, request)
    }

    /// Create a user application (blocking)
    #[cfg(feature = "sync")]
    pub fn create_user_application_blocking(
        &self,
        request: &CreateUserApplicationRequest,
    ) -> Result<UserApplicationResponse> {
        let path = "/issuing/applications/user";
        self.post_blocking(path, request)
    }

    /// Initiate a user application (blocking)
    #[cfg(feature = "sync")]
    pub fn initiate_user_application_blocking(
        &self,
        request: &InitiateUserApplicationRequest,
    ) -> Result<UserApplicationResponse> {
        let path = "/issuing/applications/user/initiate";
        self.post_blocking(path, request)
    }

    /// Get a user application by ID (blocking)
    #[cfg(feature = "sync")]
    pub fn get_user_application_blocking(&self, user_id: &Uuid) -> Result<UserApplicationResponse> {
        let path = format!("/issuing/applications/user/{user_id}");
        self.get_blocking(&path)
    }

    /// Update a user application (blocking)
    #[cfg(feature = "sync")]
    pub fn update_user_application_blocking(
        &self,
        user_id: &Uuid,
        request: &UpdateUserApplicationRequest,
    ) -> Result<UserApplicationResponse> {
        let path = format!("/issuing/applications/user/{user_id}");
        self.patch_blocking(&path, request)
    }
}
