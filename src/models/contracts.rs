//! Models for contract endpoints

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Token information in a contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractToken {
    pub address: String,
    pub balance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance_rate: Option<f64>,
}

/// ACH onramp information (AccountDetails in OpenAPI)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchOnramp {
    pub beneficiary_name: String,
    pub beneficiary_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank_address: Option<String>,
    pub account_number: String,
    pub routing_number: String,
}

/// RTP onramp information (AccountDetails in OpenAPI)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtpOnramp {
    pub beneficiary_name: String,
    pub beneficiary_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank_address: Option<String>,
    pub account_number: String,
    pub routing_number: String,
}

/// Wire onramp information (AccountDetails in OpenAPI)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireOnramp {
    pub beneficiary_name: String,
    pub beneficiary_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank_address: Option<String>,
    pub account_number: String,
    pub routing_number: String,
}

/// Onramp information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Onramp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AchOnramp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp: Option<RtpOnramp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire: Option<WireOnramp>,
}

/// Smart contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: Uuid,
    pub chain_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_address: Option<String>, // nullable, exclusive of Solana
    pub controller_address: String,
    pub proxy_address: String,
    pub deposit_address: String, // required in spec
    pub tokens: Vec<ContractToken>,
    pub contract_version: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onramp: Option<Onramp>,
}

/// Request to create a contract for a company
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCompanyContractRequest {
    pub chain_id: i64,
    pub owner_address: String,
}

/// Request to create a contract for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserContractRequest {
    pub chain_id: i64,
}

/// Request to update a contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContractRequest {
    pub onramp: bool,
}

/// Response for list of contracts (just an array of contracts)
pub type ListContractsResponse = Vec<Contract>;
