//! Models for balance endpoints

use serde::{Deserialize, Serialize};

/// Balance information response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub credit_limit: i64,
    pub pending_charges: i64,
    pub posted_charges: i64,
    pub balance_due: i64,
    pub spending_power: i64,
}
