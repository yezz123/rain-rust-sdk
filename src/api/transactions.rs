//! Transactions API
//!
//! This module provides functionality to manage transactions.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::transactions::*;
use uuid::Uuid;

impl RainClient {
    /// Get all transactions
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters to filter transactions
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Transaction>`] containing the list of transactions.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `401` - Invalid authorization
    /// - `500` - Internal server error
    #[cfg(feature = "async")]
    pub async fn list_transactions(
        &self,
        params: &ListTransactionsParams,
    ) -> Result<Vec<Transaction>> {
        let path = "/transactions";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    /// Get a transaction by its id
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The unique identifier of the transaction
    ///
    /// # Returns
    ///
    /// Returns a [`Transaction`] containing the transaction information.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `401` - Invalid authorization
    /// - `404` - Transaction not found
    /// - `500` - Internal server error
    #[cfg(feature = "async")]
    pub async fn get_transaction(&self, transaction_id: &Uuid) -> Result<Transaction> {
        let path = format!("/transactions/{transaction_id}");
        self.get(&path).await
    }

    /// Update a transaction
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The unique identifier of the transaction
    /// * `request` - The update request
    ///
    /// # Returns
    ///
    /// Returns success (204 No Content) with no response body.
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    /// - `400` - Invalid request
    /// - `401` - Invalid authorization
    /// - `404` - Transaction not found
    /// - `500` - Internal server error
    #[cfg(feature = "async")]
    pub async fn update_transaction(
        &self,
        transaction_id: &Uuid,
        request: &UpdateTransactionRequest,
    ) -> Result<()> {
        let path = format!("/transactions/{transaction_id}");
        let _: serde_json::Value = self.patch(&path, request).await?;
        Ok(())
    }

    /// Get a transaction's receipt
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The unique identifier of the transaction
    ///
    /// # Returns
    ///
    /// Returns the receipt as raw bytes (application/octet-stream).
    #[cfg(feature = "async")]
    pub async fn get_transaction_receipt(&self, transaction_id: &Uuid) -> Result<Vec<u8>> {
        let path = format!("/transactions/{transaction_id}/receipt");
        self.get_bytes(&path).await
    }

    /// Upload a transaction's receipt
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The unique identifier of the transaction
    /// * `request` - The receipt upload request
    ///
    /// # Returns
    ///
    /// Returns success (204 No Content) with no response body.
    #[cfg(feature = "async")]
    pub async fn upload_transaction_receipt(
        &self,
        transaction_id: &Uuid,
        request: &UploadReceiptRequest,
    ) -> Result<()> {
        let path = format!("/transactions/{transaction_id}/receipt");

        use reqwest::multipart::{Form, Part};
        let form = Form::new().part(
            "receipt",
            Part::bytes(request.receipt.clone()).file_name(request.file_name.clone()),
        );

        self.put_multipart_no_content(&path, form).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get all transactions (blocking)
    #[cfg(feature = "sync")]
    pub fn list_transactions_blocking(
        &self,
        params: &ListTransactionsParams,
    ) -> Result<Vec<Transaction>> {
        let path = "/transactions";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }

    /// Get a transaction by its id (blocking)
    #[cfg(feature = "sync")]
    pub fn get_transaction_blocking(&self, transaction_id: &Uuid) -> Result<Transaction> {
        let path = format!("/transactions/{transaction_id}");
        self.get_blocking(&path)
    }

    /// Update a transaction (blocking)
    #[cfg(feature = "sync")]
    pub fn update_transaction_blocking(
        &self,
        transaction_id: &Uuid,
        request: &UpdateTransactionRequest,
    ) -> Result<()> {
        let path = format!("/transactions/{transaction_id}");
        let _: serde_json::Value = self.patch_blocking(&path, request)?;
        Ok(())
    }

    /// Get a transaction's receipt (blocking)
    #[cfg(feature = "sync")]
    pub fn get_transaction_receipt_blocking(&self, transaction_id: &Uuid) -> Result<Vec<u8>> {
        let path = format!("/transactions/{transaction_id}/receipt");
        self.get_bytes_blocking(&path)
    }

    /// Upload a transaction's receipt (blocking)
    #[cfg(feature = "sync")]
    pub fn upload_transaction_receipt_blocking(
        &self,
        transaction_id: &Uuid,
        request: &UploadReceiptRequest,
    ) -> Result<()> {
        let path = format!("/transactions/{transaction_id}/receipt");

        use reqwest::blocking::multipart::{Form, Part};
        let form = Form::new().part(
            "receipt",
            Part::bytes(request.receipt.clone()).file_name(request.file_name.clone()),
        );

        self.put_multipart_blocking_no_content(&path, form)
    }
}
