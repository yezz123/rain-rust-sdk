//! Models for payment endpoints

use serde::{Deserialize, Serialize};

/// Request to initiate a payment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiatePaymentRequest {
    pub amount: i64,
    pub wallet_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<i64>,
}

/// Response from initiating a payment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiatePaymentResponse {
    pub address: String,
}
