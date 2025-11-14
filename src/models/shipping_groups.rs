//! Models for shipping group endpoints

use crate::models::common::Address;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Shipping group information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingGroup {
    pub id: Uuid,
    pub recipient_first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_phone_number: Option<String>,
    pub address: Address,
}

/// Request to create a shipping group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateShippingGroupRequest {
    pub recipient_first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_phone_number: Option<String>,
    pub address: Address,
}

/// Query parameters for listing shipping groups
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListShippingGroupsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response type for list of shipping groups
pub type ListShippingGroupsResponse = Vec<ShippingGroup>;
