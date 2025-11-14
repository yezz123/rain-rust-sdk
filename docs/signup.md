# Signing Up a Customer Guide

This guide explains how to sign up customers for your Rain Cards issuing program. The process varies slightly between Consumer and Corporate programs, with each requiring different submission methods and supporting documents.

> **Note**: This page applies to Consumer and Corporate Programs only. This is not relevant to Authorized User Program.

## Overview

When a customer enrolls in your issuing program, you must:

1. Submit an application
2. Upload supporting documents

Additionally, you can: 3. Get application updates 4. Resubmit an application (if more information is needed) 5. Test different application statuses (Sandbox only) 6. Perform additional verification (if required)

## Step 1: Submit an Application

When a customer signs up for your issuing program, you must submit an application for them. You will also need to submit specific compliance documentation with the application.

### Compliance Requirements

> **Important**: Compliance requirements may differ between contracts. Depending on the contract you have with Rain, you may be able to pass less or no compliance data when submitting an application. Read your contract carefully, or contact <platform@raincards.xyz> for help.

### Consumer Programs

For consumer programs, use the **Create a consumer application for a user** endpoint:

```rust
use rain_sdk::{RainClient, Config, Environment, AuthConfig};
use rain_sdk::models::applications::*;

let client = RainClient::new(Config::new(Environment::Dev), AuthConfig::with_api_key("your-key".to_string()))?;

let request = CreateUserApplicationRequest {
    first_name: "John".to_string(),
    last_name: "Doe".to_string(),
    birth_date: "1990-01-01".to_string(),
    national_id: "123456789".to_string(),
    country_of_issue: "US".to_string(),
    email: "john.doe@example.com".to_string(),
    ip_address: "192.168.1.1".to_string(), // Required
    occupation: "Software Engineer".to_string(),
    annual_salary: 100000,
    account_purpose: "Personal use".to_string(),
    expected_monthly_volume: 5000,
    is_terms_of_service_accepted: true,
    sumsub_share_token: Some("sumsub-token".to_string()),
    // ... address and other fields
};

let application = client.create_user_application(&request).await?;
```

### Corporate Programs

For corporate programs, use the **Create a corporate application for a company** endpoint:

> **Note**: Creating a corporate application also creates a user. We require every company on the Rain platform has at least one user. This user must have a wallet address, and this wallet will be an admin on their Rain smart contract. For your convenience, you can pass in both the user and company data to the same endpoint.

```rust
let request = CreateCompanyApplicationRequest {
    name: "Acme Corporation".to_string(),
    address: Address { /* ... */ },
    entity: EntityInfo { /* ... */ },
    initial_user: InitialUser {
        // This user will be created automatically
        wallet_address: Some("0x...".to_string()), // Required if using Rain-managed solution
        // ... other required fields
    },
    representatives: vec![/* ... */], // Required
    ultimate_beneficial_owners: vec![/* ... */], // Required
    // ... other fields
};

let application = client.create_company_application(&request).await?;
```

### Required Information

- **IP Address**: You must pass the IP address of the user in the request. If you have this handy - great! If not, you can always fetch it when the user signs up or applies for the card program.

### Response

We'll respond to the request with a user or company object. To view the status of the application, check the `applicationStatus` field. You can find more detailed information on the different application statuses in the API documentation.

### More Information

- For more information about UBOs (Ultimate Beneficial Owners), read our [Ultimate Beneficial Owner guide](https://docs.raincards.xyz).

## Step 2: Upload Supporting Documents

To upload supporting documents, use:

- **Consumer programs**: [User document upload endpoint](../examples/signup_consumer.rs)
- **Corporate programs**: [Company document upload endpoint](../examples/signup_corporate.rs)

You'll need to call the endpoint once for each document you want to upload.

### Document Requirements

- Files must be up to 20 megabytes
- See our documentation for a list of acceptable KYC documents
- Common document types include:
  - `idCard` (front and back)
  - `directorsRegistry` (for companies)
  - `proofOfAddress`
  - And more...

### Example: Uploading Documents

```rust
// Upload ID card front
let document_params = DocumentUploadParams {
    document_type: "idCard".to_string(),
    side: "front".to_string(),
    country: Some("US".to_string()),
    country_code: Some("US".to_string()),
    name: None,
    file_path: "/path/to/id_front.jpg".to_string(),
};

client.upload_user_document(&user_id, &document_params).await?;

// Upload ID card back
let document_params = DocumentUploadParams {
    document_type: "idCard".to_string(),
    side: "back".to_string(),
    // ... same fields
    file_path: "/path/to/id_back.jpg".to_string(),
};

client.upload_user_document(&user_id, &document_params).await?;
```

## Step 3 (Optional): Get Application Updates

If an application is pending, there are a few ways to stay up-to-date with their status.

### Via API

You can get the status of their application via API using:

- The user endpoint: `client.get_user(&user_id).await?`
- The company endpoint: `client.get_company(&company_id).await?`

```rust
let user = client.get_user(&user_id).await?;
println!("Application Status: {:?}", user.application_status);
```

### Via Webhook

We can also send you a webhook when their application status changes. The payload will be:

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
    "isTermsOfServiceAccepted": true,
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

## Step 4 (Optional): Resubmit an Application

If we request more information, you can update the application with the required changes. Use these endpoints, depending on the program:

- **Consumer program**: `client.update_user_application(&user_id, &request).await?`
- **Corporate program**:
  - `client.update_company_application(&company_id, &request).await?`
  - `client.update_ultimate_beneficial_owner(&company_id, &ubo_id, &request).await?`

```rust
// Example: Update user application
if let ApplicationStatus::NeedsMoreInformation = user.application_status {
    let update_request = UpdateUserApplicationRequest {
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        // ... update only the fields that need to change
    };

    let updated = client.update_user_application(&user_id, &update_request).await?;
}
```

## Step 5 (Sandbox Only): Test Different Application Statuses

As part of your integration work, you may want to test different application statuses. In the sandbox environment, we've added fixtures to enable this.

### How It Works

Simply ensure the **last name** of the user contains (case-insensitive) the application status you'd like the user to have.

### Examples

- To test **approved** status: Use names like `TestApproved`, `PersonApproved`, or simply `approved`
- To test **needsVerification** status: Use names like `TestNeedsVerification` or `needsverification`

> **Note**: `Needs Verification` will not work due to the extra whitespace. Use `needsverification` or `NeedsVerification` instead.

### Example

```rust
let request = CreateUserApplicationRequest {
    first_name: "Test".to_string(),
    last_name: "Approved".to_string(), // Contains "approved" - will be approved in sandbox
    // ... other fields
};
```

## Step 6 (Optional): Perform Additional Verification

In rare cases, a user may need to perform additional verification. If their `applicationStatus` is `needsVerification`, we'll provide an additional field in the response under `applicationExternalVerificationLink`.

### Handling Verification Links

This field is an object with:

- `url`: The URL to redirect the user to
- `params`: An object containing query parameters

### Implementation

1. Redirect the user to the given URL
2. Pass in all params as query parameters
3. Additionally, include a `redirect` param that redirects the user back to your website

```rust
if let Some(verification_link) = user.application_external_verification_link {
    let user_id = verification_link.params.get("userId").unwrap();
    let redirect_url = format!(
        "{}?userId={}&redirect=https://yourapp.com/return",
        verification_link.url,
        user_id
    );

    // Redirect user to redirect_url
    println!("Redirect user to: {}", redirect_url);
}
```

### Example Redirect URL

```sh
https://verification.raincards.xyz/verify?userId=123e4567-e89b-12d3-a456-426614174000&redirect=https://yourapp.com/return
```

## Application Statuses

The following application statuses are possible:

- `pending` - Application is being reviewed
- `approved` - Application has been approved
- `declined` - Application has been declined
- `needsMoreInformation` - More information is required
- `needsVerification` - Additional verification is required

Check the `ApplicationStatus` enum in the SDK for all possible values.

## Examples

See the following example files for complete implementations:

- [Consumer Signup Example](../examples/signup_consumer.rs)
- [Corporate Signup Example](../examples/signup_corporate.rs)

## Additional Resources

- [API Documentation](https://docs.raincards.xyz)
- [Ultimate Beneficial Owner Guide](https://docs.raincards.xyz)
- [Acceptable KYC Documents](https://docs.raincards.xyz)
