//! Models for subtenant endpoints

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Application completion link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCompletionLink {
    pub url: String,
    pub params: Value,
}

/// Subtenant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtenant {
    pub id: Uuid,
    pub name: String,
    #[serde(
        rename = "applicationCompletionLink",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_completion_link: Option<ApplicationCompletionLink>,
}

/// Request to create a subtenant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubtenantRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Request to update a subtenant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubtenantRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Response type for list of subtenants
pub type ListSubtenantsResponse = Vec<Subtenant>;
