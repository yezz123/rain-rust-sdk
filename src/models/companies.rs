//! Models for company endpoints

use crate::models::applications::UltimateBeneficialOwnerResponse;
use crate::models::common::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Company information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimate_beneficial_owners: Option<Vec<UltimateBeneficialOwnerResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status: Option<ApplicationStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_external_verification_link: Option<ApplicationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_completion_link: Option<ApplicationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_reason: Option<String>,
}

/// Request to update a company
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCompanyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

/// Query parameters for listing companies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCompaniesParams {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

/// Response for list of companies (just an array of companies)
pub type ListCompaniesResponse = Vec<Company>;
