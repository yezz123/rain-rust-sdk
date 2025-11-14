//! Models for card endpoints

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Card status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CardStatus {
    NotActivated,
    Active,
    Locked,
    Canceled,
}

/// Card type enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Physical,
    Virtual,
}

/// Limit frequency enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LimitFrequency {
    Per24HourPeriod,
    Per7DayPeriod,
    Per30DayPeriod,
    PerYearPeriod,
    AllTime,
    PerAuthorization,
}

/// Card limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardLimit {
    pub amount: i64, // Amount in cents
    pub frequency: LimitFrequency,
}

/// Card configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_card_art: Option<String>,
}

/// Shipping method enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShippingMethod {
    Standard,
    Express,
    International,
    Apc,
    UspsInternational,
}

/// Shipping address for physical cards
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingAddress {
    pub line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    pub city: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub postal_code: String,
    pub country_code: String,
    pub phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<ShippingMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// Billing address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingAddress {
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

/// Request to create a card for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCardRequest {
    pub r#type: CardType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CardStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<CardLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CardConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_shipping_group_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingAddress>,
}

/// Request to update a card
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCardRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CardStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<CardLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CardConfiguration>,
}

/// Encrypted data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub iv: String,
    pub data: String,
}

/// Request to update a card's PIN
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCardPinRequest {
    pub encrypted_pin: EncryptedData,
}

/// Response for card
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: Uuid,
    pub company_id: Uuid,
    pub user_id: Uuid,
    pub r#type: CardType,
    pub status: CardStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<CardLimit>,
    pub last4: String,
    pub expiration_month: String,
    pub expiration_year: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_wallets: Option<Vec<String>>,
}

/// Response for card secrets (encrypted PAN and CVC)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardSecrets {
    pub encrypted_pan: EncryptedData,
    pub encrypted_cvc: EncryptedData,
}

/// Response for card PIN
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPin {
    pub encrypted_pin: EncryptedData,
}

/// Response for processor details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorDetails {
    pub processor_card_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_secret: Option<String>,
}

/// Query parameters for listing cards
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCardsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CardStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response for list of cards (just an array of cards)
pub type ListCardsResponse = Vec<Card>;
