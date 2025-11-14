# Managing Cards Guide

This guide explains how to issue and manage cards once you have users in the system. Every card is linked to a specific user, and you must have at least one user before issuing a card.

This guide covers:

- How to issue cards (virtual and physical)
- How to retrieve encrypted card details securely using SessionId
- How to manage PINs
- How to use bulk shipping for multiple cards
- How to mint test tokens for collateral in sandbox environments

## Overview

Cards can be issued as virtual or physical, each with different management processes:

- **Virtual Cards**: Available for immediate use after issuance.
- **Physical Cards**: Require shipping to the user's address.

Depending on the card type, users may need to view their encrypted card details or update their PIN.

## Issuing Cards

Once a user's application is approved, you can issue them cards using the card creation endpoint. We support both virtual and physical cards, each with different setup requirements.

### Creating a Virtual Card

Virtual cards are available for immediate use. To create one, specify:

- `type` – Set this to `virtual`.
- `limit` – Define an initial spending limit, which can be adjusted later.

#### Example: Create Virtual Card

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::cards::{CreateCardRequest, CardType};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    let request = CreateCardRequest {
        user_id,
        r#type: CardType::Virtual,
        limit: Some(10000), // $100.00 in cents
        display_name: Some("John Doe".to_string()),
        shipping: None,
        bulk_shipping_group_id: None,
    };

    let card = client.create_card(&request).await?;
    println!("Virtual card created! ID: {}", card.id);
    println!("Card Status: {:?}", card.status);
    println!("Last 4: {:?}", card.last4);

    Ok(())
}
```

### Creating a Physical Card

Physical cards require shipping to the user. You must specify:

- `type` – Set this to `physical`.
- `limit` – Define an initial spending limit, which can be adjusted later.
- `shipping` – The recipient's address and phone number.

#### Example: Create Physical Card

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::cards::{CreateCardRequest, CardType, Shipping};
use rain_sdk::models::common::Address;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    let request = CreateCardRequest {
        user_id,
        r#type: CardType::Physical,
        limit: Some(50000), // $500.00 in cents
        display_name: Some("John Doe".to_string()),
        shipping: Some(Shipping {
            address: Address {
                line1: "123 Main St".to_string(),
                line2: None,
                city: "New York".to_string(),
                region: "NY".to_string(),
                postal_code: "10001".to_string(),
                country_code: "US".to_string(),
                country: None,
            },
            phone_country_code: Some("1".to_string()),
            phone_number: Some("5555555555".to_string()),
        }),
        bulk_shipping_group_id: None,
    };

    let card = client.create_card(&request).await?;
    println!("Physical card created! ID: {}", card.id);
    println!("Card will be shipped to: {} {}, {}",
        request.shipping.as_ref().unwrap().address.line1,
        request.shipping.as_ref().unwrap().address.city,
        request.shipping.as_ref().unwrap().address.region
    );

    Ok(())
}
```

### User Name Requirements for Physical Cards

Physical cards have specific character restrictions for user names:

- **User names** – The user's first name and last name must contain only Latin characters (A-Z, a-z, spaces, and hyphens). Names with non-Latin characters (such as Chinese, Arabic, Cyrillic, or accented characters like é, ñ, ü) are not supported for physical card personalization.

This restriction applies to the actual user's name that will be embossed on the physical card, which is different from shipping address requirements.

### Shipping Method Requirements

Different shipping methods have specific address format requirements:

- **DHL International** (international) – Address fields must contain only Latin characters, numbers, and basic punctuation. Non-Latin characters (Chinese, Arabic, Cyrillic, accented characters) are not supported.
- **Other methods** (standard, express, apc, uspsinternational) – Support all character sets in address fields.

For international addresses with non-Latin characters, use APC ParcelConnect (apc) or USPS International (uspsinternational) instead of DHL.

### How Spending Limits Work

The limits set to a card work on a rolling basis. The system adds up all the purchases currently "on the belt" - meaning everything purchased in the last 30 days, down to the exact second. As purchases reach the 30-day mark, they fall off the end of the belt and stop counting against your set limit.

### Display Name Character Limit

`displayName` – Used to set the name on the card. This field has a character limit of 26 characters and only allows alphanumeric characters, spaces, periods, and hyphens. By default, we use the user's full name and trim it if it exceeds the limit.

## Card Status

Cards can have one of four statuses that control their usability:

- `notActivated` – Card is issued but requires activation before use. Users cannot make transactions until the card is activated.
- `active` – Card is fully functional and can be used for transactions within its defined limits.
- `locked` – Card is temporarily disabled. All transactions are blocked, but the card can be unlocked later.
- `canceled` – Card is permanently disabled and cannot be reactivated. This action is irreversible.

You can check a card's current status using the get card endpoint, or update it using the update card endpoint.

### Example: Check Card Status

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let card_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;
    let card = client.get_card(&card_id).await?;

    println!("Card Status: {:?}", card.status);
    println!("Card Type: {:?}", card.r#type);
    println!("Last 4: {:?}", card.last4);

    Ok(())
}
```

### Activating Cards

By default, card activation is not required. If you want to enforce an activation flow, create the card with the status set to `notActivated`.

To activate a card, prompt the user to confirm the last four digits, the expiry date, or both. You can retrieve this information using the get card endpoint. If the user provides the correct details, update the card status to `active` using the update card endpoint.

#### Example: Activate a Card

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::cards::{UpdateCardRequest, CardStatus};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let card_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    // First, verify card details
    let card = client.get_card(&card_id).await?;
    println!("Card Last 4: {:?}", card.last4);
    println!("Expiry: {:?}/{:?}", card.expiration_month, card.expiration_year);

    // User confirms last 4 and expiry date
    // If correct, activate the card
    let update_request = UpdateCardRequest {
        status: Some(CardStatus::Active),
        limit: None,
    };

    let updated_card = client.update_card(&card_id, &update_request).await?;
    println!("Card activated! Status: {:?}", updated_card.status);

    Ok(())
}
```

## Bulk Shipping

Bulk shipping allows you to group multiple cards into a single shipment for efficient delivery. This reduces shipping costs and simplifies distribution, especially for organizations managing large numbers of cards.

Even in bulk shipments, each card should be:

- Individually packaged.
- Labeled with its recipient mailing address.
- Ready for easy redistribution by the bulk shipment recipient.

### Step 1: Create a Shipping Group

To initiate a bulk shipment, first create a shipping group using the Create a bulk shipping group endpoint.

This creates a unique `bulkShippingGroupId` that you will reference when creating cards to include in this shipment.

#### Example: Create Shipping Group

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::shipping_groups::CreateShippingGroupRequest;
use rain_sdk::models::common::Address;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let request = CreateShippingGroupRequest {
        recipient_first_name: "Corporate Office".to_string(),
        recipient_last_name: Some("Distribution".to_string()),
        recipient_phone_country_code: Some("1".to_string()),
        recipient_phone_number: Some("5555555555".to_string()),
        address: Address {
            line1: "123 Main Street".to_string(),
            line2: None,
            city: "Springfield".to_string(),
            region: "Arizona".to_string(),
            postal_code: "123456".to_string(),
            country_code: "US".to_string(),
            country: None,
        },
    };

    let shipping_group = client.create_shipping_group(&request).await?;
    println!("Shipping group created! ID: {}", shipping_group.id);
    println!("Bulk Shipping Group ID: {}", shipping_group.id);

    Ok(())
}
```

### Step 2: Create Cards with the Shipping Group ID

When creating each card, include the `bulkShippingGroupId` in the request payload. All cards with the same ID will be grouped for shipment.

#### Example Workflow

1. Create a shipping group → Receive `bulkShippingGroupId` = ABC123
2. Create Card A with `bulkShippingGroupId` = ABC123
3. Create Card B with `bulkShippingGroupId` = ABC123
4. Both cards will be shipped together in one bulk package.

#### Example: Create Cards in Bulk Shipping Group

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::cards::{CreateCardRequest, CardType, Shipping};
use rain_sdk::models::common::Address;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let shipping_group_id = Uuid::parse_str("abc12345-e89b-12d3-a456-426614174000")?;
    let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    // Create Card A in bulk shipping group
    let card_a_request = CreateCardRequest {
        user_id: user_id.clone(),
        r#type: CardType::Physical,
        limit: Some(10000),
        display_name: Some("Card A".to_string()),
        shipping: Some(Shipping {
            address: Address {
                line1: "123 Main St".to_string(),
                city: "New York".to_string(),
                region: "NY".to_string(),
                postal_code: "10001".to_string(),
                country_code: "US".to_string(),
                line2: None,
                country: None,
            },
            phone_country_code: Some("1".to_string()),
            phone_number: Some("5551111111".to_string()),
        }),
        bulk_shipping_group_id: Some(shipping_group_id),
    };

    let card_a = client.create_card(&card_a_request).await?;
    println!("Card A created in bulk shipping group");

    // Create Card B in same bulk shipping group
    let card_b_request = CreateCardRequest {
        user_id,
        r#type: CardType::Physical,
        limit: Some(10000),
        display_name: Some("Card B".to_string()),
        shipping: Some(Shipping {
            address: Address {
                line1: "456 Oak Ave".to_string(),
                city: "New York".to_string(),
                region: "NY".to_string(),
                postal_code: "10002".to_string(),
                country_code: "US".to_string(),
                line2: None,
                country: None,
            },
            phone_country_code: Some("1".to_string()),
            phone_number: Some("5552222222".to_string()),
        }),
        bulk_shipping_group_id: Some(shipping_group_id),
    };

    let card_b = client.create_card(&card_b_request).await?;
    println!("Card B created in bulk shipping group");
    println!("Both cards will be shipped together!");

    Ok(())
}
```

### Step 3: Understand Grouping Rules and Cutoff Times

Cards are grouped into the same shipment only if they:

- Share the same `bulkShippingGroupId`.
- Are created before **12:00 PM EST**, Monday through Friday.

#### Grouping Timing Example

| Card | Group ID | Created Time | Shipping Batch            |
| ---- | -------- | ------------ | ------------------------- |
| A    | XXX      | 11:30 AM EST | Batch 1 (together)        |
| B    | XXX      | 11:55 AM EST | Batch 1 (together)        |
| C    | XXX      | 12:15 PM EST | Batch 2 (separate)        |
| D    | YYY      | 11:45 AM EST | Separate from A, B, and C |

### Important Notes

- **Include shipping ID when creating the card**: You must include the `bulkShippingGroupId` at the time of card creation. It cannot be added or changed later.
- **Minimum cards**: A minimum of two cards is required for a shipment to qualify as a bulk group.
- **Individual packaging**: Each card is individually sealed, labeled with its specific recipient mailing address, and easy to redistribute.
- **Plan card creation time**: To ensure cards are grouped correctly, plan card creation to complete before the 12:00 PM EST cutoff.

## Viewing Encrypted Card Details

For security and compliance reasons, full card details (card number and CVC) are encrypted and should only be accessed when necessary. While the last four digits and expiry date are always accessible, full card details require a secure retrieval process using SessionId encryption.

### Security Best Practices

- **Never store decrypted card details**
- **Only request full card details when absolutely necessary**
- **Always use the latest encryption libraries to maintain security**

### Retrieving Encrypted Card Details

There are different ways to retrieve encrypted card details securely, but they all follow these steps:

1. **Generate a session ID**: Encrypt a secret using the public key provided by Rain
2. **Send an API request**: Include the encrypted session ID in the `SessionId` request header
3. **Decrypt the response**: Use the secret key to decrypt the returned card details

### SessionId Public Keys

Rain provides different public keys for different environments:

#### Development Environment

```sh
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCAP192809jZyaw62g/eTzJ3P9H
+RmT88sXUYjQ0K8Bx+rJ83f22+9isKx+lo5UuV8tvOlKwvdDS/pVbzpG7D7NO45c
0zkLOXwDHZkou8fuj8xhDO5Tq3GzcrabNLRLVz3dkx0znfzGOhnY4lkOMIdKxlQb
LuVM/dGDC9UpulF+UwIDAQAB
-----END PUBLIC KEY-----
```

#### Production Environment

```sh
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCeZ9uCoxi2XvOw1VmvVLo88TLk
GE+OO1j3fa8HhYlJZZ7CCIAsaCorrU+ZpD5PUTnmME3DJk+JyY1BB3p8XI+C5uno
QucrbxFbkM1lgR10ewz/LcuhleG0mrXL/bzUZbeJqI6v3c9bXvLPKlsordPanYBG
FZkmBPxc8QEdRgH4awIDAQAB
-----END PUBLIC KEY-----
```

### Example: Retrieve Encrypted Card Details

To retrieve encrypted card details, you need to:

1. Generate a session ID by encrypting a secret with the public key
2. Include the encrypted session ID in the `SessionId` header
3. Decrypt the response using your secret key

#### Step 1: Generate Session ID

You'll need to use an RSA encryption library to encrypt a secret with the public key. Here's a complete example using the `rsa` crate:

```rust
// Add to Cargo.toml:
// rsa = "0.9"
// base64 = "0.21"
// rand = "0.8"

use rsa::{RsaPublicKey, PaddingScheme, pkcs1::DecodeRsaPublicKey};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

// Public keys for different environments
const DEV_PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCAP192809jZyaw62g/eTzJ3P9H
+RmT88sXUYjQ0K8Bx+rJ83f22+9isKx+lo5UuV8tvOlKwvdDS/pVbzpG7D7NO45c
0zkLOXwDHZkou8fuj8xhDO5Tq3GzcrabNLRLVz3dkx0znfzGOhnY4lkOMIdKxlQb
LuVM/dGDC9UpulF+UwIDAQAB
-----END PUBLIC KEY-----"#;

const PROD_PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCeZ9uCoxi2XvOw1VmvVLo88TLk
GE+OO1j3fa8HhYlJZZ7CCIAsaCorrU+ZpD5PUTnmME3DJk+JyY1BB3p8XI+C5uno
QucrbxFbkM1lgR10ewz/LcuhleG0mrXL/bzUZbeJqI6v3c9bXvLPKlsordPanYBG
FZkmBPxc8QEdRgH4awIDAQAB
-----END PUBLIC KEY-----"#;

fn generate_session_id(
    secret: &str,
    environment: &str, // "dev" or "prod"
) -> Result<String, Box<dyn std::error::Error>> {
    // Select the appropriate public key based on environment
    let public_key_pem = match environment {
        "dev" => DEV_PUBLIC_KEY,
        "prod" => PROD_PUBLIC_KEY,
        _ => return Err("Invalid environment. Use 'dev' or 'prod'".into()),
    };

    // Parse the public key from PEM format
    let public_key = RsaPublicKey::from_pkcs1_pem(public_key_pem)?;

    // Encrypt the secret using PKCS1v15 padding
    let mut rng = rand::thread_rng();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted = public_key.encrypt(&mut rng, padding, secret.as_bytes())?;

    // Base64 encode the encrypted data
    let session_id = BASE64.encode(&encrypted);
    Ok(session_id)
}
```

#### Step 2: Retrieve Card Secrets

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let card_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    // Generate session ID using the environment-appropriate public key
    // The generate_session_id function automatically selects the correct public key
    let secret = "your-secret-key"; // Keep this secret!
    let session_id = generate_session_id(secret, "dev")?; // Use "prod" for production

    // Retrieve encrypted card secrets using the SDK method
    let card_secrets = client.get_card_secrets(&card_id, &session_id).await?;

    println!("Encrypted Card Secrets retrieved");
    println!("PAN IV: {}", card_secrets.encrypted_pan.iv);
    println!("PAN Data (encrypted): {}", card_secrets.encrypted_pan.data);
    println!("CVC IV: {}", card_secrets.encrypted_cvc.iv);
    println!("CVC Data (encrypted): {}", card_secrets.encrypted_cvc.data);

    // Important: Decrypt these values using your secret key (see Step 3)
    // Store the decrypted values securely and never log them

    Ok(())
}
```

#### Step 3: Decrypt the Response

Once you receive the encrypted card details, decrypt them using your secret key. The encryption method depends on how Rain encrypts the data - typically AES encryption is used:

```rust
// Conceptual example - implement with AES decryption
// The exact encryption method depends on Rain's implementation
// Add to Cargo.toml:
// aes-gcm = "0.10"  (or whichever cipher Rain uses)
// hex = "0.4"

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hex;

fn decrypt_card_data(encrypted_data: &str, iv: &str, secret_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the IV and encrypted data (assuming hex encoding)
    let iv_bytes = hex::decode(iv)?;
    let encrypted_bytes = hex::decode(encrypted_data)?;

    // Create cipher from secret key
    // Note: You may need to derive the key properly depending on Rain's key derivation
    let key = Aes256Gcm::new_from_slice(secret_key.as_bytes())?;
    let nonce = Nonce::from_slice(&iv_bytes);

    // Decrypt the data
    let decrypted = key.decrypt(nonce, encrypted_bytes.as_ref())?;
    let result = String::from_utf8(decrypted)?;

    Ok(result)
}

// Usage example:
// let pan = decrypt_card_data(&card_secrets.encrypted_pan.data, &card_secrets.encrypted_pan.iv, secret)?;
// let cvc = decrypt_card_data(&card_secrets.encrypted_cvc.data, &card_secrets.encrypted_cvc.iv, secret)?;
```

### Implementation Notes

The SDK provides methods for retrieving card secrets with SessionId. When implementing:

- **Store secrets securely**: Use environment variables or secret management systems (e.g., AWS Secrets Manager, HashiCorp Vault)
- **Rotate secrets regularly**: Implement a key rotation strategy
- **Use proper RSA key management**: Generate keys securely and store private keys safely
- **Implement secure key generation**: Use cryptographically secure random number generators
- **Never log secrets**: Avoid logging session IDs, secrets, or decrypted card data
- **Use HTTPS**: Always use HTTPS for all API communications

### Production Considerations

In production environments:

1. **Key Management**: Use a key management service (KMS) to store and rotate encryption keys
2. **Session Expiration**: Implement session expiration for security
3. **Audit Logging**: Log access to card secrets (without logging the actual secrets)
4. **Access Control**: Implement proper access controls to limit who can retrieve card secrets
5. **Compliance**: Ensure your implementation meets PCI DSS requirements if handling card data

## Sandbox: Mint Test Tokens for Collateral

When testing in sandbox environments, you'll need test tokens to use as collateral. This guide explains how to mint rUSD test tokens on various testnets.

### Supported Networks and Contract Addresses

Below is a table with the supported networks and the contract addresses for rUSD test token:

| Chain                   | Environment | Contract Address                             |
| ----------------------- | ----------- | -------------------------------------------- |
| Ethereum Sepolia        | Testnet     | `0x6CE0D9aEBB683AbbEc9bfbF82D35d4E92CfEC12B` |
| Optimism Sepolia        | Testnet     | `0x915F8c4a8b9fE793b3185c4186F716d7e5D891b6` |
| Arbitrum Sepolia        | Testnet     | `0xd116d4752fc50D660FB5b5c801448Ae84B4937bc` |
| Base Sepolia            | Testnet     | `0x10b5Be494C2962A7B318aFB63f0Ee30b959D000b` |
| Avalanche Fuji          | Testnet     | `0xBaFc625Fb7311Ec83D9cf4587e9D373d2ff93af8` |
| Zk Sync Sepolia         | Testnet     | `0xAe0E2f62379761044841Ce252Ad16247450a2d68` |
| Tron Shasta             | Testnet     | `0x3Ef228804f0133571b591d3a8B8eD34BD1e00834` |
| BNB Smart Chain Testnet | Testnet     | `0xdEE7360FA7B843D9312971d80e8c9Ff543AC33D1` |

### Contract Addresses Reference

For convenience in your code, here are the contract addresses as constants:

```rust
/// rUSD test token contract addresses for various testnets
pub mod testnet_contracts {
    pub const ETHEREUM_SEPOLIA: &str = "0x6CE0D9aEBB683AbbEc9bfbF82D35d4E92CfEC12B";
    pub const OPTIMISM_SEPOLIA: &str = "0x915F8c4a8b9fE793b3185c4186F716d7e5D891b6";
    pub const ARBITRUM_SEPOLIA: &str = "0xd116d4752fc50D660FB5b5c801448Ae84B4937bc";
    pub const BASE_SEPOLIA: &str = "0x10b5Be494C2962A7B318aFB63f0Ee30b959D000b";
    pub const AVALANCHE_FUJI: &str = "0xBaFc625Fb7311Ec83D9cf4587e9D373d2ff93af8";
    pub const ZK_SYNC_SEPOLIA: &str = "0xAe0E2f62379761044841Ce252Ad16247450a2d68";
    pub const TRON_SHASTA: &str = "0x3Ef228804f0133571b591d3a8B8eD34BD1e00834";
    pub const BNB_SMART_CHAIN_TESTNET: &str = "0xdEE7360FA7B843D9312971d80e8c9Ff543AC33D1";
}
```

### How to Mint rUSD on Testnets

#### EVM Networks (Ethereum, Optimism, Arbitrum, Base, Avalanche, Zk Sync, BNB Smart Chain, Tron)

To mint rUSD test tokens, follow these steps:

1. **Navigate to the block explorer** for your desired testnet (e.g., Etherscan for Ethereum Sepolia)
2. **Find the contract** using the contract address from the table above
3. **Select "Contract"** tab
4. **Select "Write Contract"** option and connect your wallet via "Connect to Web3" button
5. **Locate the `mint` function**, then specify:
   - `_amountDollars_Max100` - the amount of test tokens you want to mint (Max 100 per transaction)
6. **Click "Write"** and verify that the tokens appear in your wallet after the transaction is confirmed

##### Example: Mint rUSD on Ethereum Sepolia

```rust
// Example using the contract address constant
use testnet_contracts::ETHEREUM_SEPOLIA;

// Contract address: 0x6CE0D9aEBB683AbbEc9bfbF82D35d4E92CfEC12B
// Steps:
// 1. Go to https://sepolia.etherscan.io/address/ETHEREUM_SEPOLIA
// 2. Click "Contract" → "Write Contract"
// 3. Connect your wallet via "Connect to Web3"
// 4. Find `mint` function
// 5. Enter amount (max 100) in `_amountDollars_Max100` field
// 6. Click "Write" and confirm transaction
// 7. Wait for confirmation - tokens will appear in your wallet
```

**Block Explorer Links:**

- **Ethereum Sepolia**: [https://sepolia.etherscan.io/address/0x6CE0D9aEBB683AbbEc9bfbF82D35d4E92CfEC12B](https://sepolia.etherscan.io/address/0x6CE0D9aEBB683AbbEc9bfbF82D35d4E92CfEC12B)
- **Optimism Sepolia**: [https://sepolia-optimism.etherscan.io/address/0x915F8c4a8b9fE793b3185c4186F716d7e5D891b6](https://sepolia-optimism.etherscan.io/address/0x915F8c4a8b9fE793b3185c4186F716d7e5D891b6)
- **Arbitrum Sepolia**: [https://sepolia.arbiscan.io/address/0xd116d4752fc50D660FB5b5c801448Ae84B4937bc](https://sepolia.arbiscan.io/address/0xd116d4752fc50D660FB5b5c801448Ae84B4937bc)
- **Base Sepolia**: [https://sepolia.basescan.org/address/0x10b5Be494C2962A7B318aFB63f0Ee30b959D000b](https://sepolia.basescan.org/address/0x10b5Be494C2962A7B318aFB63f0Ee30b959D000b)
- **Avalanche Fuji**: [https://testnet.snowtrace.io/address/0xBaFc625Fb7311Ec83D9cf4587e9D373d2ff93af8](https://testnet.snowtrace.io/address/0xBaFc625Fb7311Ec83D9cf4587e9D373d2ff93af8)
- **Zk Sync Sepolia**: [https://sepolia.explorer.zksync.io/address/0xAe0E2f62379761044841Ce252Ad16247450a2d68](https://sepolia.explorer.zksync.io/address/0xAe0E2f62379761044841Ce252Ad16247450a2d68)
- **BNB Smart Chain Testnet**: [https://testnet.bscscan.com/address/0xdEE7360FA7B843D9312971d80e8c9Ff543AC33D1](https://testnet.bscscan.com/address/0xdEE7360FA7B843D9312971d80e8c9Ff543AC33D1)

#### Solana Networks

For Solana networks, go to the GitHub repository and follow its instructions for minting test tokens.

### Using Minted rUSD as Collateral

After minting, send the rUSD test tokens to your authorized user account's deposit address as outlined in the "Add Collateral to Your Account" section. Your credit limit will be updated shortly after the transaction is processed.

#### Example: Send Collateral

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    // Example workflow after minting rUSD:
    // 1. Mint rUSD on testnet using one of the contract addresses:
    //    - Ethereum Sepolia: 0x6CE0D9aEBB683AbbEc9bfbF82D35d4E92CfEC12B
    //    - Optimism Sepolia: 0x915F8c4a8b9fE793b3185c4186F716d7e5D891b6
    //    - Arbitrum Sepolia: 0xd116d4752fc50D660FB5b5c801448Ae84B4937bc
    //    - Base Sepolia: 0x10b5Be494C2962A7B318aFB63f0Ee30b959D000b
    //    - Avalanche Fuji: 0xBaFc625Fb7311Ec83D9cf4587e9D373d2ff93af8
    //    - Zk Sync Sepolia: 0xAe0E2f62379761044841Ce252Ad16247450a2d68
    //    - Tron Shasta: 0x3Ef228804f0133571b591d3a8B8eD34BD1e00834
    //    - BNB Smart Chain Testnet: 0xdEE7360FA7B843D9312971d80e8c9Ff543AC33D1
    //
    // 2. Get the user's deposit address (this would come from the user/company object)
    // 3. Send rUSD tokens to that address via blockchain transaction using your Web3 library
    // 4. Rain will automatically detect the transaction and update the balance

    println!("After minting rUSD on testnet:");
    println!("1. Mint rUSD using the contract address for your chosen testnet");
    println!("2. Get the user's deposit address");
    println!("3. Send rUSD tokens to that address via blockchain transaction");
    println!("4. Rain will automatically detect the transaction and update the balance");

    // Check user's balance after transaction is confirmed
    let balance = client.get_user_balances(&user_id).await?;
    println!("User Balance - Credit Limit: {}", balance.credit_limit);
    println!("User Balance - Spending Power: {}", balance.spending_power);

    Ok(())
}
```

### Important Notes

- **Maximum per transaction**: You can mint a maximum of 100 rUSD tokens per transaction
- **Test tokens only**: These tokens only work in testnet environments
- **Transaction confirmation**: Wait for blockchain confirmation before checking balances
- **Multiple networks**: Each network has its own contract address - use the correct one for your testnet

## Managing Card PINs

Cards can have PINs that users set and manage. The SDK provides methods to retrieve and update card PINs securely.

### Example: Update Card PIN

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::cards::{CardPin, EncryptedData};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let card_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    // PIN should be encrypted before sending
    // This is a placeholder - implement proper encryption
    let encrypted_pin = EncryptedData {
        iv: "encryption_iv_here".to_string(),
        data: "encrypted_pin_data_here".to_string(),
    };

    let pin_request = CardPin {
        encrypted_pin,
    };

    // Update the card PIN
    // Note: This requires SessionId header for security
    client.update_card_pin(&card_id, &pin_request).await?;
    println!("Card PIN updated successfully!");

    Ok(())
}
```

## Complete Example

See the following example files for complete implementations:

- [Manage Corporate Users](../examples/manage_corporate_users.rs) - For managing users before issuing cards
- [Basic Client](../examples/basic_client.rs) - For basic SDK usage

## Additional Resources

- [API Documentation](https://docs.raincards.xyz)
- [Signup Guide](./signup.md) - How to sign up customers
- [Managing Users Guide](./MANAGING_USERS.md) - How to manage corporate users
- [Card Models](../src/models/cards.rs)
- [Cards API](../src/api/cards.rs)
