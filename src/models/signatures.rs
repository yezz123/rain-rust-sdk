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
pub struct SignatureData {
    pub data: String,
    pub salt: String,
}

/// Response for payment signature (can be pending or ready)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentSignatureResponse {
    Pending {
        status: SignatureStatus,
        #[serde(rename = "retryAfter")]
        retry_after: i64,
    },
    Ready {
        status: SignatureStatus,
        signature: SignatureData,
        #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
        expires_at: Option<DateTime<Utc>>,
    },
}

/// Response for withdrawal signature (can be pending or ready)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WithdrawalSignatureResponse {
    Pending {
        status: SignatureStatus,
        #[serde(rename = "retryAfter")]
        retry_after: i64,
    },
    Ready {
        status: SignatureStatus,
        signature: SignatureData,
        #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
        expires_at: Option<DateTime<Utc>>,
    },
}

/// Query parameters for payment signature requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSignatureParams {
    #[serde(rename = "chainId", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<i64>,
    pub token: String,
    pub amount: String,
    #[serde(rename = "adminAddress")]
    pub admin_address: String,
    #[serde(rename = "isAmountNative", skip_serializing_if = "Option::is_none")]
    pub is_amount_native: Option<bool>,
    #[serde(
        rename = "rainCollateralContractId",
        skip_serializing_if = "Option::is_none"
    )]
    pub rain_collateral_contract_id: Option<Uuid>,
}

/// Query parameters for withdrawal signature requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalSignatureParams {
    #[serde(rename = "chainId", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<i64>,
    pub token: String,
    pub amount: String,
    #[serde(rename = "adminAddress")]
    pub admin_address: String,
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
    #[serde(rename = "isAmountNative", skip_serializing_if = "Option::is_none")]
    pub is_amount_native: Option<bool>,
    #[serde(
        rename = "rainCollateralContractId",
        skip_serializing_if = "Option::is_none"
    )]
    pub rain_collateral_contract_id: Option<Uuid>,
}
