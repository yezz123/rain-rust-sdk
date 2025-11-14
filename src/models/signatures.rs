//! Models for signature endpoints

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a signature
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SignatureStatus {
    Pending,
    Ready,
}

/// Signature data containing the signature and salt
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureData {
    pub data: String,
    pub salt: String,
}

/// Response for payment signature (can be pending or ready)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentSignatureResponse {
    #[serde(rename_all = "camelCase")]
    Pending {
        status: SignatureStatus,
        retry_after: i64,
    },
    #[serde(rename_all = "camelCase")]
    Ready {
        status: SignatureStatus,
        signature: SignatureData,
        #[serde(skip_serializing_if = "Option::is_none")]
        expires_at: Option<DateTime<Utc>>,
    },
}

/// Response for withdrawal signature (can be pending or ready)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WithdrawalSignatureResponse {
    #[serde(rename_all = "camelCase")]
    Pending {
        status: SignatureStatus,
        retry_after: i64,
    },
    #[serde(rename_all = "camelCase")]
    Ready {
        status: SignatureStatus,
        signature: SignatureData,
        #[serde(skip_serializing_if = "Option::is_none")]
        expires_at: Option<DateTime<Utc>>,
    },
}

/// Query parameters for payment signature requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSignatureParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<i64>,
    pub token: String,
    pub amount: String,
    pub admin_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_amount_native: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain_collateral_contract_id: Option<Uuid>,
}

/// Query parameters for withdrawal signature requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalSignatureParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<i64>,
    pub token: String,
    pub amount: String,
    pub admin_address: String,
    pub recipient_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_amount_native: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain_collateral_contract_id: Option<Uuid>,
}
