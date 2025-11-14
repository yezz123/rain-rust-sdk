# Managing Users Guide

This guide explains how to manage users in your Rain Cards issuing program for Corporate Programs.

> **Note**: This page applies to Corporate Programs only.

## Overview

Once a company's application has been approved, you can manage additional users in your corporate program. This includes:

1. **Add user** - Add new users to the company
2. **Deactivate user** - Set users as active or inactive
3. **Delete user** - Delete user information (also cancels all their cards)

## Add User

To add a user, you will need different endpoints depending on your program type:

- **Corporate programs**: Use the company user creation endpoint
- **Authorized User programs**: Use the general user creation endpoint

### Corporate Programs

Once a company's application has been approved, you can add new users to the company. This step only requires the user's first name, last name, and email. You can also provide a wallet address to associate with this user.

Users will also go through our compliance process. You can receive a webhook to know when the user's compliance status changes. The payload will be the `userId` and their new compliance status.

#### Example: Create a Company User

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::users::CreateCompanyUserRequest;
use rain_sdk::models::common::Address;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let company_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    let request = CreateCompanyUserRequest {
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        email: "jane.smith@company.com".to_string(),
        is_terms_of_service_accepted: true,
        birth_date: Some("1990-05-15".to_string()),
        wallet_address: Some("0xabcdef1234567890abcdef1234567890abcdef12".to_string()),
        solana_address: None,
        address: Some(Address {
            line1: "789 Corporate Ave".to_string(),
            line2: None,
            city: "San Francisco".to_string(),
            region: "CA".to_string(),
            postal_code: "94105".to_string(),
            country_code: "US".to_string(),
            country: None,
        }),
        phone_country_code: Some("1".to_string()),
        phone_number: Some("5559876543".to_string()),
    };

    let user = client.create_company_user(&company_id, &request).await?;
    println!("User created! ID: {}", user.id);
    println!("Application Status: {:?}", user.application_status);

    Ok(())
}
```

#### Required Fields

- `first_name` - The user's first name
- `last_name` - The user's last name
- `email` - The user's email address
- `is_terms_of_service_accepted` - Whether the user has accepted the terms of service

#### Optional Fields

- `birth_date` - The user's birth date (ISO format: "YYYY-MM-DD")
- `wallet_address` - The user's EVM wallet address
- `solana_address` - The user's Solana wallet address
- `address` - The user's physical address
- `phone_country_code` - The user's phone country code
- `phone_number` - The user's phone number

#### Webhook Payload

When a user's compliance status changes, you'll receive a webhook with the following payload:

```json
{
  "resource": "user",
  "action": "updated",
  "body": {
    "id": "string",
    "firstName": "string",
    "lastName": "string",
    "email": "string",
    "isActive": true,
    "applicationStatus": "string",
    "applicationExternalVerificationLink": {
      "url": "string",
      "params": {
        "userId": "string"
      }
    }
  }
}
```

### Authorized User Programs

When a customer signs up for your issuing program, you must create a user for them. You'll need their legal first and last name and an email address. Generally, you must perform due diligence and compliance with the user before creating the user entity.

#### Example: Create an Authorized User

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::users::CreateUserRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let request = CreateUserRequest {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        wallet_address: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
        solana_address: None,
        address: None,
        phone_country_code: None,
        phone_number: None,
    };

    let user = client.create_user(&request).await?;
    println!("User created! ID: {}", user.id);

    Ok(())
}
```

## List Users

You can list all users for a company using the list users endpoint with a company filter.

### Example: List Company Users

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::users::ListUsersParams;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let company_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    let params = ListUsersParams {
        company_id: Some(company_id),
        cursor: None,
        limit: Some(20),
    };

    let users = client.list_users(&params).await?;

    println!("Found {} users", users.len());
    for user in &users {
        println!("  - {} {} (ID: {}, Active: {})",
            user.first_name,
            user.last_name,
            user.id,
            user.is_active
        );
    }

    Ok(())
}
```

### Query Parameters

- `company_id` (optional) - Filter users by company ID
- `cursor` (optional) - Pagination cursor
- `limit` (optional) - Maximum number of users to return

## Get User

Retrieve a specific user by their ID.

### Example: Get User Details

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;
    let user = client.get_user(&user_id).await?;

    println!("User Details:");
    println!("  ID: {}", user.id);
    println!("  Name: {} {}", user.first_name, user.last_name);
    println!("  Email: {}", user.email);
    println!("  Active: {}", user.is_active);
    println!("  Application Status: {:?}", user.application_status);

    Ok(())
}
```

## Deactivate User

We allow you to set users as active or inactive. You can update the user with the desired value in the `isActive` parameter. This will change their status, and you will see the change when you fetch users through the API.

### Example: Deactivate a User

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::users::UpdateUserRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    // Deactivate the user
    let update_request = UpdateUserRequest {
        first_name: None,
        last_name: None,
        email: None,
        is_active: Some(false), // Set to false to deactivate
        is_terms_of_service_accepted: None,
        address: None,
        phone_country_code: None,
        phone_number: None,
        wallet_address: None,
        solana_address: None,
        tron_address: None,
        stellar_address: None,
    };

    let updated_user = client.update_user(&user_id, &update_request).await?;
    println!("User deactivated! Active Status: {}", updated_user.is_active);

    // Verify the user is inactive
    let user_check = client.get_user(&user_id).await?;
    if !user_check.is_active {
        println!("Confirmed: User is now inactive");
    }

    Ok(())
}
```

### Example: Reactivate a User

```rust
// Reactivate the user
let reactivate_request = UpdateUserRequest {
    first_name: None,
    last_name: None,
    email: None,
    is_active: Some(true), // Set to true to reactivate
    is_terms_of_service_accepted: None,
    address: None,
    phone_country_code: None,
    phone_number: None,
    wallet_address: None,
    solana_address: None,
    tron_address: None,
    stellar_address: None,
};

let reactivated_user = client.update_user(&user_id, &reactivate_request).await?;
println!("User reactivated! Active Status: {}", reactivated_user.is_active);
```

## Delete User

Submit the `userId` to the delete user endpoint to delete their information. This will also set the status of all cards of that user to cancelled.

> **Warning**: Deleting a user is a destructive operation that cannot be undone. All cards associated with the user will be cancelled.

### Example: Delete a User

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    // Delete the user
    // This will also cancel all cards associated with this user
    client.delete_user(&user_id).await?;
    println!("✅ User deleted successfully!");

    Ok(())
}
```

### Important Notes

- **Irreversible Action**: Once a user is deleted, their information cannot be recovered.
- **Card Cancellation**: All cards associated with the deleted user will automatically be set to cancelled status.
- **Use with Caution**: Consider deactivating users instead of deleting them if you may need to restore access later.

## Complete Example

See the [Manage Corporate Users example](../examples/manage_corporate_users.rs) for a complete workflow that demonstrates:

1. Adding a new user to a company
2. Listing all users for a company
3. Getting user details
4. Deactivating a user
5. Reactivating a user
6. Deleting a user (commented out for safety)

## Additional Resources

- [API Documentation](https://docs.raincards.xyz)
- [Corporate Signup Guide](./signup.md)
- [User Models](../src/models/users.rs)
- [Companies API](../src/api/companies.rs)
- [Users API](../src/api/users.rs)
