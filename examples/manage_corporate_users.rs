//! Example: Managing Corporate Users
//!
//! This example demonstrates how to manage users in a corporate program:
//! 1. Add a new user to a company (after company application is approved)
//! 2. Deactivate a user
//! 3. Delete a user
//!
//! Note: This applies to Corporate Programs only.

use rain_sdk::models::users::*;
use rain_sdk::{AuthConfig, Config, Environment, RainClient};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    // Assume we have a company that has been approved
    let company_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")?;

    // Step 1: Add a new user to the company
    // Note: This can only be done after the company's application has been approved
    println!("Step 1: Adding a new user to the company...");

    let create_user_request = CreateCompanyUserRequest {
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        email: "jane.smith@company.com".to_string(),
        is_terms_of_service_accepted: true,
        birth_date: Some("1990-05-15".to_string()),
        wallet_address: Some("0xabcdef1234567890abcdef1234567890abcdef12".to_string()),
        solana_address: None,
        address: Some(rain_sdk::models::common::Address {
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

    let new_user = client
        .create_company_user(&company_id, &create_user_request)
        .await?;
    println!("✅ User created successfully!");
    println!("User ID: {}", new_user.id);
    println!("Name: {} {}", new_user.first_name, new_user.last_name);
    println!("Email: {}", new_user.email);
    println!("Is Active: {}", new_user.is_active);
    println!("Application Status: {:?}", new_user.application_status);

    // Note: The user will go through compliance process
    // You can receive webhooks when the user's compliance status changes
    // Webhook payload will include userId and their new compliance status

    // Step 2: List users for the company
    println!("\nStep 2: Listing all users for the company...");

    let list_params = ListUsersParams {
        company_id: Some(company_id),
        cursor: None,
        limit: Some(20),
    };

    let users = client.list_users(&list_params).await?;
    println!("Found {} users", users.len());
    for user in &users {
        println!(
            "  - {} {} (ID: {}, Active: {})",
            user.first_name, user.last_name, user.id, user.is_active
        );
    }

    // Step 3: Get a specific user
    println!("\nStep 3: Getting user details...");
    let user_id = new_user.id;

    let user = client.get_user(&user_id).await?;
    println!("User Details:");
    println!("  ID: {}", user.id);
    println!("  Name: {} {}", user.first_name, user.last_name);
    println!("  Email: {}", user.email);
    println!("  Active: {}", user.is_active);
    println!("  Application Status: {:?}", user.application_status);

    // Step 4: Deactivate a user
    println!("\nStep 4: Deactivating the user...");

    let update_request = UpdateUserRequest {
        first_name: None,
        last_name: None,
        email: None,
        is_active: Some(false), // Deactivate the user
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
    println!("✅ User deactivated successfully!");
    println!("User Active Status: {}", updated_user.is_active);

    // Verify the user is inactive
    let user_check = client.get_user(&user_id).await?;
    if !user_check.is_active {
        println!("Confirmed: User is now inactive");
    }

    // Step 5: Reactivate the user (optional)
    println!("\nStep 5: Reactivating the user...");

    let reactivate_request = UpdateUserRequest {
        first_name: None,
        last_name: None,
        email: None,
        is_active: Some(true), // Reactivate the user
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
    println!("✅ User reactivated successfully!");
    println!("User Active Status: {}", reactivated_user.is_active);

    // Step 6: Delete a user
    // Note: This will also set the status of all cards of that user to cancelled
    println!("\nStep 6: Deleting the user...");
    println!("Warning: This will delete the user and cancel all their cards!");

    // Uncomment the line below to actually delete the user
    // client.delete_user(&user_id).await?;
    // println!("✅ User deleted successfully!");

    println!("(Deletion skipped in this example - uncomment to execute)");

    // Alternative: Delete a different user by ID if needed
    // let user_to_delete_id = Uuid::parse_str("another-user-id")?;
    // client.delete_user(&user_to_delete_id).await?;

    println!("\n✅ All user management operations completed!");

    Ok(())
}
