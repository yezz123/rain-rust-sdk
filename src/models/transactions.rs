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
#[serde(rename_all = "camelCase")]
pub struct SpendTransaction {
    pub amount: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    pub receipt: bool,
    pub merchant_name: String,
    pub merchant_category: String,
    pub merchant_category_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_merchant_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_merchant_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_merchant_category: Option<String>,
    pub card_id: Uuid,
    pub card_type: CardType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    pub user_id: Uuid,
    pub user_first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_last_name: Option<String>,
    pub user_email: String,
    pub status: SpendTransactionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declined_reason: Option<String>,
    pub authorized_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<String>,
}

/// Collateral transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralTransaction {
    pub amount: f64,
    pub currency: String,
    pub chain_id: i64,
    pub wallet_address: String,
    pub transaction_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<DateTime<Utc>>,
}

/// Payment transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTransaction {
    pub amount: i64,
    pub currency: String,
    pub status: PaymentTransactionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<String>,
}

/// Fee transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeTransaction {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<Uuid>,
    #[serde(
        rename = "type",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_transaction_types"
    )]
    pub transaction_type: Option<Vec<TransactionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[serde(rename_all = "camelCase")]
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
