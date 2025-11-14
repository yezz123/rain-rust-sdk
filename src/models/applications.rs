//! Models for application endpoints

use crate::models::common::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Company Application Models
// ============================================================================

/// Request to create a company application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
///
/// This request supports three verification methods (oneOf in OpenAPI spec):
/// 1. **Using Sumsub Share Token**: Provide only `sumsub_share_token`
/// 2. **Using Persona Share Token**: Provide only `persona_share_token`
/// 3. **Using API**: Provide full person data (all `IssuingApplicationPerson` fields)
///
/// Exactly one verification method must be provided. The API will validate this at runtime.
///
/// # Verification Methods
///
/// ## Sumsub Share Token
/// ```rust
/// use rain_sdk::models::applications::CreateUserApplicationRequest;
///
/// CreateUserApplicationRequest {
///     sumsub_share_token: Some("your-sumsub-token".to_string()),
///     persona_share_token: None,
///     id: None,
///     first_name: None,
///     last_name: None,
///     birth_date: None,
///     national_id: None,
///     country_of_issue: None,
///     email: None,
///     phone_country_code: None,
///     phone_number: None,
///     address: None,
///     ip_address: "127.0.0.1".to_string(),
///     occupation: "Engineer".to_string(),
///     annual_salary: "100000".to_string(),
///     account_purpose: "Business".to_string(),
///     expected_monthly_volume: "5000".to_string(),
///     is_terms_of_service_accepted: true,
///     wallet_address: None,
///     solana_address: None,
///     tron_address: None,
///     stellar_address: None,
///     chain_id: None,
///     contract_address: None,
///     source_key: None,
///     has_existing_documents: None,
/// };
/// ```
///
/// ## Persona Share Token
/// ```rust
/// use rain_sdk::models::applications::CreateUserApplicationRequest;
///
/// CreateUserApplicationRequest {
///     sumsub_share_token: None,
///     persona_share_token: Some("your-persona-token".to_string()),
///     id: None,
///     first_name: None,
///     last_name: None,
///     birth_date: None,
///     national_id: None,
///     country_of_issue: None,
///     email: None,
///     phone_country_code: None,
///     phone_number: None,
///     address: None,
///     ip_address: "127.0.0.1".to_string(),
///     occupation: "Engineer".to_string(),
///     annual_salary: "100000".to_string(),
///     account_purpose: "Business".to_string(),
///     expected_monthly_volume: "5000".to_string(),
///     is_terms_of_service_accepted: true,
///     wallet_address: None,
///     solana_address: None,
///     tron_address: None,
///     stellar_address: None,
///     chain_id: None,
///     contract_address: None,
///     source_key: None,
///     has_existing_documents: None,
/// };
/// ```
///
/// ## Full API (IssuingApplicationPerson)
/// ```rust
/// use rain_sdk::models::applications::CreateUserApplicationRequest;
/// use rain_sdk::models::common::Address;
///
/// CreateUserApplicationRequest {
///     sumsub_share_token: None,
///     persona_share_token: None,
///     id: None,
///     first_name: Some("John".to_string()),
///     last_name: Some("Doe".to_string()),
///     birth_date: Some("2000-01-01".to_string()),
///     national_id: Some("123456789".to_string()),
///     country_of_issue: Some("US".to_string()),
///     email: Some("john@example.com".to_string()),
///     phone_country_code: Some("1".to_string()),
///     phone_number: Some("5555555555".to_string()),
///     address: Some(Address {
///         line1: "123 Main St".to_string(),
///         line2: None,
///         city: "New York".to_string(),
///         region: "NY".to_string(),
///         postal_code: "10001".to_string(),
///         country_code: "US".to_string(),
///         country: None,
///     }),
///     ip_address: "127.0.0.1".to_string(),
///     occupation: "Engineer".to_string(),
///     annual_salary: "100000".to_string(),
///     account_purpose: "Business".to_string(),
///     expected_monthly_volume: "5000".to_string(),
///     is_terms_of_service_accepted: true,
///     wallet_address: None,
///     solana_address: None,
///     tron_address: None,
///     stellar_address: None,
///     chain_id: None,
///     contract_address: None,
///     source_key: None,
///     has_existing_documents: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserApplicationRequest {
    // Verification method - exactly one must be provided
    // Option 1: Using Sumsub Share Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sumsub_share_token: Option<String>,
    // Option 2: Using Persona Share Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona_share_token: Option<String>,

    // Option 3: Using API - Full IssuingApplicationPerson fields
    /// The initiated application's Rain ID, if an application was previously initiated for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    /// The person's first name (max 50 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The person's last name (max 50 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The person's birth date (ISO date format: YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    /// The person's national id, 9-digit SSN if country of issue is US
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id: Option<String>,
    /// The person's country of issue of their national id (2-letter country code)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_issue: Option<String>,
    /// The person's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The user's phone country code (1-3 digits)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
    /// The user's phone number (1-15 digits)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The person's address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::models::common::Address>,

    // Required fields (always present)
    /// This user's IP address
    pub ip_address: String,
    /// The user's occupation
    pub occupation: String,
    /// The user's annual salary
    pub annual_salary: String,
    /// The purpose of the user's account
    pub account_purpose: String,
    /// The amount of money the user expects to spend each month
    pub expected_monthly_volume: String,
    /// Whether the user has accepted the terms of service
    pub is_terms_of_service_accepted: bool,

    // Optional fields
    /// This user's EVM address; either walletAddress or solanaAddress or tronAddress or stellarAddress is required if using a Rain-managed solution, but not required otherwise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    /// This user's Solana address; either walletAddress or solanaAddress or tronAddress or stellarAddress is required if using a Rain-managed solution, but not required otherwise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solana_address: Option<String>,
    /// This user's Tron address; either walletAddress or solanaAddress or tronAddress or stellarAddress is required if using a Rain-managed solution, but not required otherwise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tron_address: Option<String>,
    /// This user's Stellar address; either walletAddress or solanaAddress or tronAddress or stellarAddress is required if using a Rain-managed solution, but not required otherwise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stellar_address: Option<String>,
    /// If using external collateral contracts, the chain id of the user's external collateral contract; not required if using Rain's collateral contracts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    /// If using external collateral contracts, the address of the user's external collateral contract; not required if using Rain's collateral contracts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    /// Unique identifier for where this user is coming from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key: Option<String>,
    /// Whether or not to use existing documents for additional verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_existing_documents: Option<bool>,
}

/// Request to initiate a user application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
