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
pub struct Dispute {
    pub id: Uuid,
    #[serde(rename = "transactionId")]
    pub transaction_id: Uuid,
    pub status: DisputeStatus,
    #[serde(rename = "textEvidence", skip_serializing_if = "Option::is_none")]
    pub text_evidence: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "resolvedAt", skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Query parameters for listing disputes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDisputesParams {
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Request to update a dispute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDisputeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DisputeStatus>,
    #[serde(rename = "textEvidence", skip_serializing_if = "Option::is_none")]
    pub text_evidence: Option<String>,
}

/// Request to create a dispute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDisputeRequest {
    #[serde(rename = "textEvidence", skip_serializing_if = "Option::is_none")]
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
