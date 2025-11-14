//! Models for transaction endpoints

use crate::models::cards::CardType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Transaction type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Spend,
    Collateral,
    Payment,
    Fee,
}

/// Spend transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpendTransactionStatus {
    Pending,
    Reversed,
    Declined,
    Completed,
}

/// Payment transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentTransactionStatus {
    Pending,
    Completed,
}

/// Spend transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendTransaction {
    pub amount: i64,
    pub currency: String,
    #[serde(rename = "localAmount", skip_serializing_if = "Option::is_none")]
    pub local_amount: Option<i64>,
    #[serde(rename = "localCurrency", skip_serializing_if = "Option::is_none")]
    pub local_currency: Option<String>,
    #[serde(rename = "authorizedAmount", skip_serializing_if = "Option::is_none")]
    pub authorized_amount: Option<i64>,
    #[serde(
        rename = "authorizationMethod",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    pub receipt: bool,
    #[serde(rename = "merchantName")]
    pub merchant_name: String,
    #[serde(rename = "merchantCategory")]
    pub merchant_category: String,
    #[serde(rename = "merchantCategoryCode")]
    pub merchant_category_code: String,
    #[serde(rename = "merchantId", skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(
        rename = "enrichedMerchantIcon",
        skip_serializing_if = "Option::is_none"
    )]
    pub enriched_merchant_icon: Option<String>,
    #[serde(
        rename = "enrichedMerchantName",
        skip_serializing_if = "Option::is_none"
    )]
    pub enriched_merchant_name: Option<String>,
    #[serde(
        rename = "enrichedMerchantCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub enriched_merchant_category: Option<String>,
    #[serde(rename = "cardId")]
    pub card_id: Uuid,
    #[serde(rename = "cardType")]
    pub card_type: CardType,
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    #[serde(rename = "userFirstName")]
    pub user_first_name: String,
    #[serde(rename = "userLastName", skip_serializing_if = "Option::is_none")]
    pub user_last_name: Option<String>,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    pub status: SpendTransactionStatus,
    #[serde(rename = "declinedReason", skip_serializing_if = "Option::is_none")]
    pub declined_reason: Option<String>,
    #[serde(rename = "authorizedAt")]
    pub authorized_at: String,
    #[serde(rename = "postedAt", skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<String>,
}

/// Collateral transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralTransaction {
    pub amount: f64,
    pub currency: String,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(rename = "postedAt", skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<DateTime<Utc>>,
}

/// Payment transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransaction {
    pub amount: i64,
    pub currency: String,
    pub status: PaymentTransactionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(rename = "chainId", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<i64>,
    #[serde(rename = "walletAddress", skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(rename = "transactionHash", skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(rename = "postedAt", skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<String>,
}

/// Fee transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeTransaction {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(rename = "postedAt", skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<DateTime<Utc>>,
}

/// Transaction (discriminated union based on type)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum Transaction {
    #[serde(rename = "spend")]
    Spend {
        id: Uuid,
        #[serde(flatten)]
        spend: SpendTransaction,
    },
    #[serde(rename = "collateral")]
    Collateral {
        id: Uuid,
        #[serde(flatten)]
        collateral: CollateralTransaction,
    },
    #[serde(rename = "payment")]
    Payment {
        id: Uuid,
        #[serde(flatten)]
        payment: PaymentTransaction,
    },
    #[serde(rename = "fee")]
    Fee {
        id: Uuid,
        #[serde(flatten)]
        fee: FeeTransaction,
    },
}

/// Query parameters for listing transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTransactionsParams {
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(rename = "cardId", skip_serializing_if = "Option::is_none")]
    pub card_id: Option<Uuid>,
    #[serde(
        rename = "type",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_transaction_types"
    )]
    pub transaction_type: Option<Vec<TransactionType>>,
    #[serde(rename = "transactionHash", skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "authorizedBefore", skip_serializing_if = "Option::is_none")]
    pub authorized_before: Option<DateTime<Utc>>,
    #[serde(rename = "authorizedAfter", skip_serializing_if = "Option::is_none")]
    pub authorized_after: Option<DateTime<Utc>>,
    #[serde(rename = "postedBefore", skip_serializing_if = "Option::is_none")]
    pub posted_before: Option<DateTime<Utc>>,
    #[serde(rename = "postedAfter", skip_serializing_if = "Option::is_none")]
    pub posted_after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

fn serialize_transaction_types<S>(
    types: &Option<Vec<TransactionType>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;

    match types {
        Some(ref vec) => {
            let mut seq = serializer.serialize_seq(Some(vec.len()))?;
            for item in vec {
                let s = match item {
                    TransactionType::Spend => "spend",
                    TransactionType::Collateral => "collateral",
                    TransactionType::Payment => "payment",
                    TransactionType::Fee => "fee",
                };
                seq.serialize_element(s)?;
            }
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}

/// Request to update a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTransactionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Request to upload a receipt
#[derive(Debug, Clone)]
pub struct UploadReceiptRequest {
    pub receipt: Vec<u8>,
    pub file_name: String,
}

/// Response type for list of transactions
pub type ListTransactionsResponse = Vec<Transaction>;
