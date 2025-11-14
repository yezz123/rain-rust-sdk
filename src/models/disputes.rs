//! Models for dispute endpoints

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a dispute
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DisputeStatus {
    Pending,
    InReview,
    Accepted,
    Rejected,
    Canceled,
}

/// Dispute information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dispute {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub status: DisputeStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_evidence: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Query parameters for listing disputes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDisputesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Request to update a dispute
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDisputeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DisputeStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_evidence: Option<String>,
}

/// Request to create a dispute
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDisputeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_evidence: Option<String>,
}

/// Request to upload dispute evidence
#[derive(Debug, Clone)]
pub struct UploadDisputeEvidenceRequest {
    pub name: String,
    pub evidence_type: String,
    pub file: Vec<u8>,
}

/// Response type for list of disputes
pub type ListDisputesResponse = Vec<Dispute>;
