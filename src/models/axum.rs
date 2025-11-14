//! Axum wrapper types for deserialization and OpenAPI schema generation
//!
//! This module provides wrapper types that implement `Deserialize` for Axum's `Json` extractor
//! and `` for OpenAPI documentation generation with `utoipa`.
//!
//! These types wrap the SDK's request types, allowing them to be used directly
//! with Axum's JSON extractor while maintaining compatibility with the SDK's internal types.

#[cfg(feature = "axum")]
use crate::models::applications::{
    CreateCompanyApplicationRequest, CreateUserApplicationRequest, InitiateUserApplicationRequest,
    UpdateCompanyApplicationRequest, UpdateUserApplicationRequest,
};
#[cfg(feature = "axum")]
use crate::models::balances::BalanceResponse;
#[cfg(feature = "axum")]
use crate::models::cards::{Card, CreateCardRequest, ListCardsParams, UpdateCardRequest};
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

// ============================================================================
// Request Wrapper Types (for Deserialization)
// ============================================================================

// Applications
/// Wrapper for InitiateUserApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InitiateUserApplicationRequestWrapper(pub InitiateUserApplicationRequest);

/// Wrapper for CreateUserApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateUserApplicationRequestWrapper(pub CreateUserApplicationRequest);

/// Wrapper for UpdateUserApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateUserApplicationRequestWrapper(pub UpdateUserApplicationRequest);

/// Wrapper for CreateCompanyApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateCompanyApplicationRequestWrapper(pub CreateCompanyApplicationRequest);

/// Wrapper for UpdateCompanyApplicationRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateCompanyApplicationRequestWrapper(pub UpdateCompanyApplicationRequest);

// Cards
/// Wrapper for CreateCardRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateCardRequestWrapper(pub CreateCardRequest);

/// Wrapper for UpdateCardRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateCardRequestWrapper(pub UpdateCardRequest);

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
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateCompanyUserRequestWrapper(pub CreateCompanyUserRequest);

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
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateShippingGroupRequestWrapper(pub CreateShippingGroupRequest);

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
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateUserRequestWrapper(pub UpdateUserRequest);

// ============================================================================
// Response Wrapper Types (for Serialization and OpenAPI)
// ============================================================================

// Applications - responses are typically already in models, just re-export for

// Balances
/// Wrapper for BalanceResponse that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BalanceResponseWrapper(pub BalanceResponse);

// Cards
/// Wrapper for Card that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CardWrapper(pub Card);

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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DisputeWrapper(pub Dispute);

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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ShippingGroupWrapper(pub ShippingGroup);

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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TransactionWrapper(pub Transaction);

// Users
/// Wrapper for User that implements
#[cfg(feature = "axum")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserWrapper(pub User);

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
