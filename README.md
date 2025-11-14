# Rain SDK

![Rain SDK](https://app.ashbyhq.com/api/images/org-theme-wordmark/dc58a004-9283-4567-8b9e-641f9135d176/f93d4997-00f5-421f-9ba6-5979c6300ded/1cc77dcf-1f1e-4813-bc12-1db61148263d.png)

[![Documentation](https://docs.rs/rain-sdk/badge.svg)](https://docs.rs/rain-sdk)
[![Crates.io](https://img.shields.io/crates/v/rain-sdk.svg)](https://crates.io/crates/rain-sdk)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.60+-black.svg)](https://www.rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/rain-sdk)
[![CI](https://github.com/yezz123/rain-rust-sdk/actions/workflows/ci.yaml/badge.svg)](https://github.com/yezz123/rain-rust-sdk/actions/workflows/ci.yaml)
[![dependency status](https://deps.rs/repo/github/yezz123/rain-rust-sdk/status.svg)](https://deps.rs/repo/github/yezz123/rain-rust-sdk)

---

A modern, type-safe Rust SDK for the [Rain Cards API](https://raincards.xyz).

## Features

- ✅ **100% API Coverage**: All 71 non-deprecated endpoints fully implemented
- ✅ **OpenAPI Aligned**: Fully aligned with the official OpenAPI specification
- ✅ **Async and Sync Support**: Use async/await or blocking operations via feature flags
- ✅ **Type Safety**: Strongly typed models for all API endpoints with proper serialization
- ✅ **API Key Authentication**: Simple API key-based authentication
- ✅ **Comprehensive Error Handling**: Detailed error types with HTTP status codes and context
- ✅ **Production Ready**: Well-tested, documented, and validated

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rain-sdk = { version = "0.1.0", features = ["async"] }
```

For blocking/sync operations:

```toml
[dependencies]
rain-sdk = { version = "0.1.0", features = ["sync"] }
```

## Quick Start

### Async Example

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::applications::InitiateUserApplicationRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration for dev environment
    let config = Config::new(Environment::Dev);

    // Create authentication config with API key
    let auth = AuthConfig::with_api_key("your-api-key".to_string());

    // Create the client
    let client = RainClient::new(config, auth)?;

    // Initiate a user application
    let request = InitiateUserApplicationRequest {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        wallet_address: None,
    };

    let application = client.initiate_user_application(&request).await?;
    println!("Application ID: {}", application.id);

    Ok(())
}
```

### Sync Example

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::applications::InitiateUserApplicationRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let request = InitiateUserApplicationRequest {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        wallet_address: None,
    };

    let application = client.initiate_user_application_blocking(&request)?;
    println!("Application ID: {}", application.id);

    Ok(())
}
```

## Authentication

The SDK supports API key authentication:

```rust
let auth = AuthConfig::with_api_key("your-api-key".to_string());
```

## Examples

See the [examples](examples/) directory for usage examples:

- **[Basic Client](./examples/basic_client.rs)** - Initialize a client and make simple API calls
- **[Initiate User Application](./examples/initiate_user_application.rs)** - Initiate a user application
- **[Consumer Signup](./examples/signup_consumer.rs)** - Complete consumer signup workflow
- **[Corporate Signup](./examples/signup_corporate.rs)** - Complete corporate signup workflow
- **[Manage Corporate Users](./examples/manage_corporate_users.rs)** - Manage users in corporate programs (add, deactivate, delete)

Run an example:

```bash
cargo run --example signup_consumer --features async
```

## API Coverage

The SDK provides **100% coverage** of the Rain Issuing API, with all 71 non-deprecated endpoints fully implemented and aligned with the OpenAPI specification.

| Category | Endpoints | Description |
|----------|-----------|-------------|
| **Applications** | | |
| Company Applications | Create, Get, Update | Manage company applications and ultimate beneficial owners (UBOs) |
| | Update UBO | Update UBO information |
| | Upload Documents | Upload company and UBO verification documents |
| User Applications | Create | Create user applications (supports Sumsub, Persona, and full API verification) |
| | Initiate, Get, Update | Manage user application lifecycle |
| | Upload Documents | Upload user verification documents |
| **Cards** | | |
| | List, Get | Retrieve card information |
| | Create | Create virtual and physical cards for users |
| | Update | Update card status, limits, and billing addresses |
| | Get Secrets | Retrieve encrypted PAN, CVC, and PIN |
| | Get Processor Details | Get processor card ID and time-based secrets |
| | Update PIN | Update card PIN |
| **Companies** | | |
| | List, Get, Update | Manage company information |
| | Create Users | Add users to companies |
| | Get Balances | Retrieve company credit balances |
| | Create Charges | Apply custom fees to companies |
| | Get/Create Contracts | Manage smart contracts |
| | Initiate Payments | Process company payments |
| | Get Signatures | Get payment and withdrawal signatures |
| **Users** | | |
| | List, Get, Create, Update, Delete | Full user lifecycle management |
| | Get Balances | Retrieve user credit balances |
| | Create Charges | Apply custom fees to users |
| | Get/Create Contracts | Manage user smart contracts |
| | Create Cards | Issue cards to users |
| | Initiate Payments | Process user payments |
| | Get Signatures | Get payment and withdrawal signatures |
| **Transactions** | | |
| | List, Get | Retrieve transaction history |
| | Update | Update transaction memos |
| | Create Disputes | Create disputes for transactions |
| | Get/Upload Receipts | Manage transaction receipts |
| **Disputes** | | |
| | List, Get | Retrieve dispute information |
| | Update | Update dispute status and evidence |
| | Get/Upload Evidence | Manage dispute file evidence |
| **Contracts** | | |
| | Get | Retrieve contracts for companies and users |
| | Create | Deploy new smart contracts |
| | Update | Update contract settings (onramp configuration) |
| **Payments** | | |
| | Initiate | Process payments for companies, users, and tenants |
| | Get Address | Retrieve payment destination addresses |
| **Signatures** | | |
| | Get Payment Signatures | Retrieve payment signatures for smart contract interactions |
| | Get Withdrawal Signatures | Retrieve withdrawal signatures for smart contract interactions |
| **Balances** | | |
| | Get Tenant Balances | Retrieve overall tenant credit information |
| | Get Company Balances | Retrieve company-specific balances |
| | Get User Balances | Retrieve user-specific balances |
| **Keys** | | |
| | Create | Generate new API keys |
| | Delete | Revoke API keys |
| **Shipping Groups** | | |
| | Create, List, Get | Manage bulk shipping groups for physical cards |
| **Subtenants** | | |
| | Create, List, Get, Update | Manage subtenant accounts |
| **Reports** | | |
| | Get Reports | Download tenant reports (CSV, JSON, or ZIP format) |
| **Webhooks** | | |
| | List, Get | Retrieve webhook information with filtering |

## Error Handling

The SDK uses a comprehensive error type system with detailed HTTP status codes:

```rust
use rain_sdk::error::RainError;

match client.get_user_application(&user_id).await {
    Ok(application) => println!("Success: {:?}", application),
    Err(RainError::ApiError { status, response }) => {
        println!("API error (status {}): {}", status, response);
        // Common status codes:
        // - 400: Invalid request
        // - 401: Invalid authorization
        // - 403: Forbidden
        // - 404: Not found
        // - 409: Conflict
        // - 423: Locked
        // - 500: Internal server error
    },
    Err(RainError::HttpError(err)) => println!("HTTP error: {}", err),
    Err(RainError::AuthError(msg)) => println!("Auth error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Configuration

### Environment Selection

```rust
// Dev (default)
let config = Config::new(Environment::Dev);

// Production
let config = Config::new(Environment::Production);

// Custom URL
let config = Config::new(Environment::Custom(
    url::Url::parse("https://api.custom.com/v1")?
));
```

### Custom Configuration

```rust
let config = Config::new(Environment::Dev)
    .with_timeout(60)  // 60 second timeout
    .with_user_agent("my-app/1.0".to_string())
    .with_logging(true);
```

## Features

- `default`: Enables async support with rustls-tls
- `async`: Async/await support (requires tokio)
- `sync`: Blocking/synchronous operations
- `rustls-tls`: Use rustls for TLS (default)
- `native-tls`: Use native TLS implementation
- `gzip`: Enable gzip compression
- `json`: JSON serialization support (enabled by default)

## Documentation

- **[Signup Guide](./docs/signup.md)** - Complete guide for signing up customers (Consumer and Corporate programs)
- **[Managing Users Guide](./docs/managing_users.md)** - Guide for managing users in corporate programs (add, deactivate, delete)
- **[Managing Cards Guide](./docs/managing_cards.md)** - Guide for issuing and managing cards (virtual, physical, bulk shipping, encrypted details)

## License

Licensed under the MIT license ([LICENSE](LICENSE)).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
