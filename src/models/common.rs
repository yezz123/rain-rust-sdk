//! Common types and models for the Rain SDK

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Address structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub country_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Application link with parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationLink {
    pub url: String,
    pub params: ApplicationLinkParams,
}

/// Application link parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationLinkParams {
    pub user_id: Uuid,
}

/// Person information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonInfo {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>, // ISO date format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_issue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub address: Address,
}

/// Application status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApplicationStatus {
    Pending,
    Approved,
    Rejected,
    InReview,
}

/// Document type for company documents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompanyDocumentType {
    DirectorsRegistry,
    StateRegistry,
    IncumbencyCert,
    ProofOfAddress,
    TrustAgreement,
    InformationStatement,
    IncorporationCert,
    IncorporationArticles,
    ShareholderRegistry,
    GoodStandingCert,
    PowerOfAttorney,
    Other,
}

/// Document type for user/UBO documents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserDocumentType {
    IdCard,
    Passport,
    Drivers,
    ResidencePermit,
    UtilityBill,
    Selfie,
    VideoSelfie,
    ProfileImage,
    IdDocPhoto,
    Agreement,
    Contract,
    DriversTranslation,
    InvestorDoc,
    VehicleRegistrationCertificate,
    IncomeSource,
    PaymentMethod,
    BankCard,
    CovidVaccinationForm,
    Other,
}

/// Document side enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentSide {
    Front,
    Back,
}
