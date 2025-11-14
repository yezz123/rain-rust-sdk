//! Models for charge endpoints

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request to create a charge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChargeRequest {
    pub amount: i64, // Amount in cents, must be >= 1
    pub description: String,
}

/// Charge information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charge {
    pub id: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: String, // ISO 8601 date string
    pub amount: i64,
    pub description: String,
}
