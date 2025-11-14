//! Models for application endpoints

use crate::models::common::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Company Application Models
// ============================================================================

/// Request to create a company application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompanyApplicationRequest {
    pub initial_user: InitialUser,
    pub name: String,
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key: Option<String>,
    pub entity: EntityInfo,
    pub representatives: Vec<Representative>,
    pub ultimate_beneficial_owners: Vec<UltimateBeneficialOwner>,
}

/// Initial user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub national_id: String,
    pub country_of_issue: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solana_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tron_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stellar_address: Option<String>,
    pub ip_address: String,
    pub is_terms_of_service_accepted: bool,
}

/// Entity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub description: String,
    pub industry: String,
    pub registration_number: String,
    pub tax_id: String,
    pub website: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_spend: Option<String>,
}

/// Entity update information (all fields optional for updates)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityUpdateInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_spend: Option<String>,
}

/// Representative information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Representative {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub national_id: String,
    pub country_of_issue: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub address: Address,
}

/// Ultimate beneficial owner information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UltimateBeneficialOwner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub national_id: String,
    pub country_of_issue: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub address: Address,
}

/// Response for company application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyApplicationResponse {
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

/// Ultimate beneficial owner response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UltimateBeneficialOwnerResponse {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status: Option<ApplicationStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_external_verification_link: Option<ApplicationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_completion_link: Option<ApplicationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_reason: Option<String>,
}

/// Request to update a company application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCompanyApplicationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<EntityUpdateInfo>,
}

/// Request to update an ultimate beneficial owner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUltimateBeneficialOwnerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_issue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

// ============================================================================
// User Application Models
// ============================================================================

/// Request to create a user application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserApplicationRequest {
    pub ip_address: String,
    pub occupation: String,
    pub annual_salary: String,
    pub account_purpose: String,
    pub expected_monthly_volume: String,
    pub is_terms_of_service_accepted: bool,
    pub sumsub_share_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solana_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tron_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stellar_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_existing_documents: Option<bool>,
}

/// Request to initiate a user application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateUserApplicationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
}

/// Response for user application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserApplicationResponse {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terms_of_service_accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solana_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status: Option<ApplicationStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_external_verification_link: Option<ApplicationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_completion_link: Option<ApplicationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_reason: Option<String>,
}

/// Request to update a user application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserApplicationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_issue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_salary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_monthly_volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terms_of_service_accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_existing_documents: Option<bool>,
}

// ============================================================================
// Document Upload Models
// ============================================================================

/// Parameters for document upload
#[derive(Debug, Clone)]
pub struct DocumentUploadParams {
    pub document_type: String,
    pub side: String,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub name: Option<String>, // Only for company documents
    pub file_path: String,
}
