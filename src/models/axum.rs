//! Axum wrapper types for deserialization and OpenAPI schema generation
//!
//! This module provides wrapper types that implement `Deserialize` for Axum's `Json` extractor
//! and `` for OpenAPI documentation generation with `utoipa`.
//!
//! These types wrap the SDK's request types, allowing them to be used directly
//! with Axum's JSON extractor while maintaining compatibility with the SDK's internal types.

#[cfg(feature = "axum")]
use crate::models::applications::{
    CompanyApplicationResponse, CreateCompanyApplicationRequest, CreateUserApplicationRequest,
    InitiateUserApplicationRequest, UpdateCompanyApplicationRequest, UpdateUserApplicationRequest,
    UserApplicationResponse,
};
#[cfg(feature = "axum")]
use crate::models::balances::BalanceResponse;
#[cfg(feature = "axum")]
use crate::models::cards::{
    Card, CardSecrets, CreateCardRequest, ListCardsParams, UpdateCardRequest,
};
#[cfg(feature = "axum")]
use crate::models::charges::CreateChargeRequest;
#[cfg(feature = "axum")]
use crate::models::companies::{Company, ListCompaniesParams, UpdateCompanyRequest};
#[cfg(feature = "axum")]
use crate::models::contracts::{
    Contract, CreateCompanyContractRequest, CreateUserContractRequest, UpdateContractRequest,
};
#[cfg(feature = "axum")]
use crate::models::disputes::{
    CreateDisputeRequest, Dispute, ListDisputesParams, UpdateDisputeRequest,
};
#[cfg(feature = "axum")]
use crate::models::keys::{CreateKeyRequest, Key};
#[cfg(feature = "axum")]
use crate::models::payments::{InitiatePaymentRequest, InitiatePaymentResponse};
#[cfg(feature = "axum")]
use crate::models::reports::GetReportParams;
#[cfg(feature = "axum")]
use crate::models::shipping_groups::{
    CreateShippingGroupRequest, ListShippingGroupsParams, ShippingGroup,
};
#[cfg(feature = "axum")]
use crate::models::signatures::{
    PaymentSignatureParams, PaymentSignatureResponse, WithdrawalSignatureParams,
    WithdrawalSignatureResponse,
};
#[cfg(feature = "axum")]
use crate::models::subtenants::{
    ApplicationCompletionLink, CreateSubtenantRequest, Subtenant, UpdateSubtenantRequest,
};
#[cfg(feature = "axum")]
use crate::models::transactions::{ListTransactionsParams, Transaction, UpdateTransactionRequest};
#[cfg(feature = "axum")]
use crate::models::users::CreateCompanyUserRequest;
#[cfg(feature = "axum")]
use crate::models::users::{CreateUserRequest, ListUsersParams, UpdateUserRequest, User};
#[cfg(feature = "axum")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "axum")]
use utoipa::ToSchema;

// ============================================================================
// Request Wrapper Types (for Deserialization)
// ============================================================================

// Applications
/// Wrapper for InitiateUserApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InitiateUserApplicationRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<InitiateUserApplicationRequestWrapper> for InitiateUserApplicationRequest {
    fn from(wrapper: InitiateUserApplicationRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner).unwrap_or_else(|_| {
            // Fallback to default if deserialization fails
            InitiateUserApplicationRequest {
                first_name: None,
                last_name: None,
                email: None,
                wallet_address: None,
            }
        })
    }
}

/// Wrapper for CreateUserApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateUserApplicationRequestWrapper(pub CreateUserApplicationRequest);

/// Wrapper for UpdateUserApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserApplicationRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<UpdateUserApplicationRequestWrapper> for UpdateUserApplicationRequest {
    fn from(wrapper: UpdateUserApplicationRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner).unwrap_or_else(|_| UpdateUserApplicationRequest {
            first_name: None,
            last_name: None,
            birth_date: None,
            national_id: None,
            country_of_issue: None,
            address: None,
            ip_address: None,
            occupation: None,
            annual_salary: None,
            account_purpose: None,
            expected_monthly_volume: None,
            is_terms_of_service_accepted: None,
            has_existing_documents: None,
        })
    }
}

/// Wrapper for CreateCompanyApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCompanyApplicationRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<CreateCompanyApplicationRequestWrapper> for CreateCompanyApplicationRequest {
    fn from(wrapper: CreateCompanyApplicationRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner)
            .expect("Failed to deserialize CreateCompanyApplicationRequest")
    }
}

/// Wrapper for UpdateCompanyApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCompanyApplicationRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<UpdateCompanyApplicationRequestWrapper> for UpdateCompanyApplicationRequest {
    fn from(wrapper: UpdateCompanyApplicationRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner)
            .expect("Failed to deserialize UpdateCompanyApplicationRequest")
    }
}

// Cards
/// Wrapper for CreateCardRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCardRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<CreateCardRequestWrapper> for CreateCardRequest {
    fn from(wrapper: CreateCardRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner).expect("Failed to deserialize CreateCardRequest")
    }
}

/// Wrapper for UpdateCardRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCardRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<UpdateCardRequestWrapper> for UpdateCardRequest {
    fn from(wrapper: UpdateCardRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner).expect("Failed to deserialize UpdateCardRequest")
    }
}

// Charges
/// Wrapper for CreateChargeRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateChargeRequestWrapper(pub CreateChargeRequest);

// Companies
/// Wrapper for UpdateCompanyRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateCompanyRequestWrapper(pub UpdateCompanyRequest);

/// Wrapper for CreateCompanyUserRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCompanyUserRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<CreateCompanyUserRequestWrapper> for CreateCompanyUserRequest {
    fn from(wrapper: CreateCompanyUserRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner)
            .expect("Failed to deserialize CreateCompanyUserRequest")
    }
}

// Contracts
/// Wrapper for CreateCompanyContractRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateCompanyContractRequestWrapper(pub CreateCompanyContractRequest);

/// Wrapper for CreateUserContractRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateUserContractRequestWrapper(pub CreateUserContractRequest);

/// Wrapper for UpdateContractRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateContractRequestWrapper(pub UpdateContractRequest);

// Disputes
/// Wrapper for CreateDisputeRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateDisputeRequestWrapper(pub CreateDisputeRequest);

/// Wrapper for UpdateDisputeRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateDisputeRequestWrapper(pub UpdateDisputeRequest);

// Keys
/// Wrapper for CreateKeyRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateKeyRequestWrapper(pub CreateKeyRequest);

// Payments
/// Wrapper for InitiatePaymentRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InitiatePaymentRequestWrapper(pub InitiatePaymentRequest);

// Shipping Groups
/// Wrapper for CreateShippingGroupRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateShippingGroupRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<CreateShippingGroupRequestWrapper> for CreateShippingGroupRequest {
    fn from(wrapper: CreateShippingGroupRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner)
            .expect("Failed to deserialize CreateShippingGroupRequest")
    }
}

// Subtenants
/// Wrapper for CreateSubtenantRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateSubtenantRequestWrapper(pub CreateSubtenantRequest);

/// Wrapper for UpdateSubtenantRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateSubtenantRequestWrapper(pub UpdateSubtenantRequest);

// Transactions
/// Wrapper for UpdateTransactionRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateTransactionRequestWrapper(pub UpdateTransactionRequest);

// Users
/// Wrapper for CreateUserRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateUserRequestWrapper(pub CreateUserRequest);

/// Wrapper for UpdateUserRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<UpdateUserRequestWrapper> for UpdateUserRequest {
    fn from(wrapper: UpdateUserRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner).expect("Failed to deserialize UpdateUserRequest")
    }
}

// ============================================================================
// Response Wrapper Types (for Serialization and OpenAPI)
// ============================================================================

// Applications
/// Wrapper for UserApplicationResponse that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserApplicationResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<UserApplicationResponse> for UserApplicationResponseWrapper {
    fn from(response: UserApplicationResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for CompanyApplicationResponse that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyApplicationResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<CompanyApplicationResponse> for CompanyApplicationResponseWrapper {
    fn from(response: CompanyApplicationResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

// Balances
/// Wrapper for BalanceResponse that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BalanceResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<BalanceResponse> for BalanceResponseWrapper {
    fn from(response: BalanceResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

// Cards
/// Wrapper for Card that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CardWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<Card> for CardWrapper {
    fn from(response: Card) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for CardSecrets that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CardSecretsWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<CardSecrets> for CardSecretsWrapper {
    fn from(response: CardSecrets) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

// Companies
/// Wrapper for Company that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CompanyWrapper(pub Company);

// Contracts
/// Wrapper for Contract that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContractWrapper(pub Contract);

// Disputes
/// Wrapper for Dispute that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DisputeWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<Dispute> for DisputeWrapper {
    fn from(response: Dispute) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

// Keys
/// Wrapper for Key that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct KeyWrapper(pub Key);

// Payments
/// Wrapper for InitiatePaymentResponse that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InitiatePaymentResponseWrapper(pub InitiatePaymentResponse);

/// Wrapper for PaymentSignatureResponse that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PaymentSignatureResponseWrapper(pub PaymentSignatureResponse);

/// Wrapper for WithdrawalSignatureResponse that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WithdrawalSignatureResponseWrapper(pub WithdrawalSignatureResponse);

// Shipping Groups
/// Wrapper for ShippingGroup that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ShippingGroupWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<ShippingGroup> for ShippingGroupWrapper {
    fn from(response: ShippingGroup) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

// Subtenants
/// Wrapper for Subtenant that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubtenantWrapper(pub Subtenant);

/// Wrapper for ApplicationCompletionLink that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApplicationCompletionLinkWrapper(pub ApplicationCompletionLink);

// Transactions
/// Wrapper for Transaction that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<Transaction> for TransactionWrapper {
    fn from(response: Transaction) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

// Users
/// Wrapper for User that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<User> for UserWrapper {
    fn from(response: User) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

// ============================================================================
// Query Parameter Wrapper Types
// ============================================================================

/// Wrapper for ListCardsParams that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListCardsParamsWrapper(pub ListCardsParams);

/// Wrapper for ListCompaniesParams that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListCompaniesParamsWrapper(pub ListCompaniesParams);

/// Wrapper for ListDisputesParams that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListDisputesParamsWrapper(pub ListDisputesParams);

/// Wrapper for ListShippingGroupsParams that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListShippingGroupsParamsWrapper(pub ListShippingGroupsParams);

/// Wrapper for ListTransactionsParams that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListTransactionsParamsWrapper(pub ListTransactionsParams);

/// Wrapper for ListUsersParams that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListUsersParamsWrapper(pub ListUsersParams);

/// Wrapper for PaymentSignatureParams that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PaymentSignatureParamsWrapper(pub PaymentSignatureParams);

/// Wrapper for WithdrawalSignatureParams that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WithdrawalSignatureParamsWrapper(pub WithdrawalSignatureParams);

/// Wrapper for GetReportParams that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetReportParamsWrapper(pub GetReportParams);
