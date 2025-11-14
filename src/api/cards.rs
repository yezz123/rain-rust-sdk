//! Cards API
//!
//! This module provides functionality to manage cards.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::cards::*;
use uuid::Uuid;

impl RainClient {
    /// Get all cards for a user or company
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering cards
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Card>`] containing the list of cards.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::cards::ListCardsParams;
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let user_id = Uuid::new_v4();
    /// let params = ListCardsParams {
    ///     user_id: Some(user_id),
    ///     company_id: None,
    ///     status: None,
    ///     cursor: None,
    ///     limit: Some(20),
    /// };
    /// let cards = client.list_cards(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn list_cards(&self, params: &ListCardsParams) -> Result<ListCardsResponse> {
        let mut path = "/issuing/cards".to_string();
        let mut query_parts = Vec::new();

        if let Some(ref company_id) = params.company_id {
            query_parts.push(format!("companyId={company_id}"));
        }
        if let Some(ref user_id) = params.user_id {
            query_parts.push(format!("userId={user_id}"));
        }
        if let Some(ref status) = params.status {
            let status_str = serde_json::to_string(status)?;
            query_parts.push(format!("status={}", status_str.trim_matches('"')));
        }
        if let Some(ref cursor) = params.cursor {
            query_parts.push(format!("cursor={cursor}"));
        }
        if let Some(limit) = params.limit {
            query_parts.push(format!("limit={limit}"));
        }

        if !query_parts.is_empty() {
            path.push('?');
            path.push_str(&query_parts.join("&"));
        }

        self.get(&path).await
    }

    /// Get a card by its ID
    ///
    /// # Arguments
    ///
    /// * `card_id` - The unique identifier of the card
    ///
    /// # Returns
    ///
    /// Returns a [`Card`] containing the card information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let card_id = Uuid::new_v4();
    /// let card = client.get_card(&card_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_card(&self, card_id: &Uuid) -> Result<Card> {
        let path = format!("/issuing/cards/{card_id}");
        self.get(&path).await
    }

    /// Update a card
    ///
    /// # Arguments
    ///
    /// * `card_id` - The unique identifier of the card
    /// * `request` - The update request
    ///
    /// # Returns
    ///
    /// Returns a [`Card`] containing the updated card information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::cards::{UpdateCardRequest, CardStatus};
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let card_id = Uuid::new_v4();
    /// let request = UpdateCardRequest {
    ///     status: Some(CardStatus::Active),
    ///     limit: None,
    ///     billing: None,
    ///     configuration: None,
    /// };
    /// let card = client.update_card(&card_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn update_card(&self, card_id: &Uuid, request: &UpdateCardRequest) -> Result<Card> {
        let path = format!("/issuing/cards/{card_id}");
        self.patch(&path, request).await
    }

    /// Get a card's encrypted data (PAN and CVC)
    ///
    /// # Arguments
    ///
    /// * `card_id` - The unique identifier of the card
    /// * `session_id` - The encrypted session ID
    ///
    /// # Returns
    ///
    /// Returns a [`CardSecrets`] containing the encrypted PAN and CVC.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let card_id = Uuid::new_v4();
    /// let session_id = "your-session-id".to_string();
    /// let secrets = client.get_card_secrets(&card_id, &session_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_card_secrets(&self, card_id: &Uuid, session_id: &str) -> Result<CardSecrets> {
        let path = format!("/issuing/cards/{card_id}/secrets");
        self.get_with_headers(&path, vec![("SessionId", session_id)])
            .await
    }

    /// Get processor details of a card
    ///
    /// # Arguments
    ///
    /// * `card_id` - The unique identifier of the card
    ///
    /// # Returns
    ///
    /// Returns a [`ProcessorDetails`] containing the processor card ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let card_id = Uuid::new_v4();
    /// let details = client.get_card_processor_details(&card_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_card_processor_details(&self, card_id: &Uuid) -> Result<ProcessorDetails> {
        let path = format!("/issuing/cards/{card_id}/processorDetails");
        self.get(&path).await
    }

    /// Get a card's PIN
    ///
    /// # Arguments
    ///
    /// * `card_id` - The unique identifier of the card
    /// * `session_id` - The encrypted session ID
    ///
    /// # Returns
    ///
    /// Returns a [`CardPin`] containing the encrypted PIN.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let card_id = Uuid::new_v4();
    /// let session_id = "your-session-id".to_string();
    /// let pin = client.get_card_pin(&card_id, &session_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_card_pin(&self, card_id: &Uuid, session_id: &str) -> Result<CardPin> {
        let path = format!("/issuing/cards/{card_id}/pin");
        self.get_with_headers(&path, vec![("SessionId", session_id)])
            .await
    }

    /// Update a card's PIN
    ///
    /// # Arguments
    ///
    /// * `card_id` - The unique identifier of the card
    /// * `request` - The PIN update request with encrypted PIN
    /// * `session_id` - The encrypted session ID
    ///
    /// # Returns
    ///
    /// Returns success (200 OK) with no response body.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::cards::{UpdateCardPinRequest, EncryptedData};
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let card_id = Uuid::new_v4();
    /// let session_id = "your-session-id".to_string();
    /// let request = UpdateCardPinRequest {
    ///     encrypted_pin: EncryptedData {
    ///         iv: "initialization-vector".to_string(),
    ///         data: "encrypted-data".to_string(),
    ///     },
    /// };
    /// client.update_card_pin(&card_id, &request, &session_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn update_card_pin(
        &self,
        card_id: &Uuid,
        request: &UpdateCardPinRequest,
        session_id: &str,
    ) -> Result<()> {
        let path = format!("/issuing/cards/{card_id}/pin");
        // Use a dummy deserializable type for empty response
        let _: serde_json::Value = self
            .put_with_headers(&path, request, vec![("SessionId", session_id)])
            .await?;
        Ok(())
    }

    /// Create a card for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user
    /// * `request` - The card creation request
    ///
    /// # Returns
    ///
    /// Returns a [`Card`] containing the created card information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::cards::{CreateCardRequest, CardType};
    /// use uuid::Uuid;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let user_id = Uuid::new_v4();
    /// let request = CreateCardRequest {
    ///     r#type: CardType::Virtual,
    ///     status: None,
    ///     limit: None,
    ///     configuration: None,
    ///     shipping: None,
    ///     bulk_shipping_group_id: None,
    ///     billing: None,
    /// };
    /// let card = client.create_user_card(&user_id, &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_user_card(
        &self,
        user_id: &Uuid,
        request: &CreateCardRequest,
    ) -> Result<Card> {
        let path = format!("/issuing/users/{user_id}/cards");
        self.post(&path, request).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get all cards for a user or company (blocking)
    #[cfg(feature = "sync")]
    pub fn list_cards_blocking(&self, params: &ListCardsParams) -> Result<Vec<Card>> {
        let mut path = "/issuing/cards".to_string();
        let mut query_parts = Vec::new();

        if let Some(ref company_id) = params.company_id {
            query_parts.push(format!("companyId={company_id}"));
        }
        if let Some(ref user_id) = params.user_id {
            query_parts.push(format!("userId={user_id}"));
        }
        if let Some(ref status) = params.status {
            let status_str = serde_json::to_string(status)?;
            query_parts.push(format!("status={}", status_str.trim_matches('"')));
        }
        if let Some(ref cursor) = params.cursor {
            query_parts.push(format!("cursor={cursor}"));
        }
        if let Some(limit) = params.limit {
            query_parts.push(format!("limit={limit}"));
        }

        if !query_parts.is_empty() {
            path.push('?');
            path.push_str(&query_parts.join("&"));
        }

        self.get_blocking(&path)
    }

    /// Get a card by its ID (blocking)
    #[cfg(feature = "sync")]
    pub fn get_card_blocking(&self, card_id: &Uuid) -> Result<Card> {
        let path = format!("/issuing/cards/{card_id}");
        self.get_blocking(&path)
    }

    /// Update a card (blocking)
    #[cfg(feature = "sync")]
    pub fn update_card_blocking(
        &self,
        card_id: &Uuid,
        request: &UpdateCardRequest,
    ) -> Result<Card> {
        let path = format!("/issuing/cards/{card_id}");
        self.patch_blocking(&path, request)
    }

    /// Create a card for a user (blocking)
    #[cfg(feature = "sync")]
    pub fn create_user_card_blocking(
        &self,
        user_id: &Uuid,
        request: &CreateCardRequest,
    ) -> Result<Card> {
        let path = format!("/issuing/users/{user_id}/cards");
        self.post_blocking(&path, request)
    }
}
