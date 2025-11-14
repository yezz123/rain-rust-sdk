//! Models for key endpoints

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request to create a key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKeyRequest {
    pub name: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,
}

/// Key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,
}
