//! Disputes API
//!
//! This module provides functionality to manage disputes.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::disputes::*;
use uuid::Uuid;

impl RainClient {
    /// Get all disputes
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters to filter disputes
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Dispute>`] containing the list of disputes.
    #[cfg(feature = "async")]
    pub async fn list_disputes(&self, params: &ListDisputesParams) -> Result<Vec<Dispute>> {
        let path = "/issuing/disputes";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get(&full_path).await
    }

    /// Get a dispute by its id
    ///
    /// # Arguments
    ///
    /// * `dispute_id` - The unique identifier of the dispute
    ///
    /// # Returns
    ///
    /// Returns a [`Dispute`] containing the dispute information.
    #[cfg(feature = "async")]
    pub async fn get_dispute(&self, dispute_id: &Uuid) -> Result<Dispute> {
        let path = format!("/issuing/disputes/{dispute_id}");
        self.get(&path).await
    }

    /// Update a dispute
    ///
    /// # Arguments
    ///
    /// * `dispute_id` - The unique identifier of the dispute
    /// * `request` - The update request
    ///
    /// # Returns
    ///
    /// Returns success (204 No Content) with no response body.
    #[cfg(feature = "async")]
    pub async fn update_dispute(
        &self,
        dispute_id: &Uuid,
        request: &UpdateDisputeRequest,
    ) -> Result<()> {
        let path = format!("/issuing/disputes/{dispute_id}");
        let _: serde_json::Value = self.patch(&path, request).await?;
        Ok(())
    }

    /// Get a dispute's file evidence
    ///
    /// # Arguments
    ///
    /// * `dispute_id` - The unique identifier of the dispute
    ///
    /// # Returns
    ///
    /// Returns the file evidence as raw bytes (application/octet-stream).
    #[cfg(feature = "async")]
    pub async fn get_dispute_evidence(&self, dispute_id: &Uuid) -> Result<Vec<u8>> {
        let path = format!("/issuing/disputes/{dispute_id}/evidence");
        self.get_bytes(&path).await
    }

    /// Upload a file as evidence for a dispute
    ///
    /// # Arguments
    ///
    /// * `dispute_id` - The unique identifier of the dispute
    /// * `request` - The evidence upload request
    ///
    /// # Returns
    ///
    /// Returns success (204 No Content) with no response body.
    #[cfg(feature = "async")]
    pub async fn upload_dispute_evidence(
        &self,
        dispute_id: &Uuid,
        request: &UploadDisputeEvidenceRequest,
    ) -> Result<()> {
        let path = format!("/issuing/disputes/{dispute_id}/evidence");

        use reqwest::multipart::{Form, Part};
        let form = Form::new()
            .text("name", request.name.clone())
            .text("type", request.evidence_type.clone())
            .part(
                "evidence",
                Part::bytes(request.file.clone()).file_name(request.name.clone()),
            );

        self.put_multipart_no_content(&path, form).await
    }

    /// Create a dispute for a transaction
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The unique identifier of the transaction
    /// * `request` - The dispute creation request
    ///
    /// # Returns
    ///
    /// Returns a [`Dispute`] containing the created dispute information.
    #[cfg(feature = "async")]
    pub async fn create_transaction_dispute(
        &self,
        transaction_id: &Uuid,
        request: &CreateDisputeRequest,
    ) -> Result<Dispute> {
        let path = format!("/issuing/transactions/{transaction_id}/disputes");
        self.post(&path, request).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get all disputes (blocking)
    #[cfg(feature = "sync")]
    pub fn list_disputes_blocking(&self, params: &ListDisputesParams) -> Result<Vec<Dispute>> {
        let path = "/issuing/disputes";
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query_string}")
        };
        self.get_blocking(&full_path)
    }

    /// Get a dispute by its id (blocking)
    #[cfg(feature = "sync")]
    pub fn get_dispute_blocking(&self, dispute_id: &Uuid) -> Result<Dispute> {
        let path = format!("/issuing/disputes/{dispute_id}");
        self.get_blocking(&path)
    }

    /// Update a dispute (blocking)
    #[cfg(feature = "sync")]
    pub fn update_dispute_blocking(
        &self,
        dispute_id: &Uuid,
        request: &UpdateDisputeRequest,
    ) -> Result<()> {
        let path = format!("/issuing/disputes/{dispute_id}");
        let _: serde_json::Value = self.patch_blocking(&path, request)?;
        Ok(())
    }

    /// Get a dispute's file evidence (blocking)
    #[cfg(feature = "sync")]
    pub fn get_dispute_evidence_blocking(&self, dispute_id: &Uuid) -> Result<Vec<u8>> {
        let path = format!("/issuing/disputes/{dispute_id}/evidence");
        self.get_bytes_blocking(&path)
    }

    /// Upload a file as evidence for a dispute (blocking)
    #[cfg(feature = "sync")]
    pub fn upload_dispute_evidence_blocking(
        &self,
        dispute_id: &Uuid,
        request: &UploadDisputeEvidenceRequest,
    ) -> Result<()> {
        let path = format!("/issuing/disputes/{dispute_id}/evidence");

        use reqwest::blocking::multipart::{Form, Part};
        let form = Form::new()
            .text("name", request.name.clone())
            .text("type", request.evidence_type.clone())
            .part(
                "evidence",
                Part::bytes(request.file.clone()).file_name(request.name.clone()),
            );

        let url = self.build_url(&path)?;
        let response = self.put_multipart_blocking_no_content(&path, form)?;
        Ok(response)
    }

    /// Create a dispute for a transaction (blocking)
    #[cfg(feature = "sync")]
    pub fn create_transaction_dispute_blocking(
        &self,
        transaction_id: &Uuid,
        request: &CreateDisputeRequest,
    ) -> Result<Dispute> {
        let path = format!("/issuing/transactions/{transaction_id}/disputes");
        self.post_blocking(&path, request)
    }
}
