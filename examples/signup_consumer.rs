//! Example: Signing up a consumer customer
//!
//! This example demonstrates the workflow for signing up a consumer customer:
//! 1. Submit an application
//! 2. Upload supporting documents
//! 3. Get application updates (optional)
//! 4. Resubmit an application if needed (optional)
//! 5. Perform additional verification if needed (optional)

use rain_sdk::models::applications::*;
use rain_sdk::models::common::*;
use rain_sdk::{AuthConfig, Config, Environment, RainClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    // Step 1: Submit an application
    println!("Step 1: Submitting consumer application...");

    // First, initiate the application with basic info
    let initiate_request = InitiateUserApplicationRequest {
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("john.doe@example.com".to_string()),
        wallet_address: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
    };

    let initiated_application = client.initiate_user_application(&initiate_request).await?;
    println!(
        "Application initiated! User ID: {}",
        initiated_application.id
    );

    // Then, complete the application with additional information
    // Note: In a real scenario, you would get the sumsub_share_token from SumSub integration
    let application_request = CreateUserApplicationRequest {
        ip_address: "192.168.1.1".to_string(), // Required: User's IP address
        occupation: "Software Engineer".to_string(),
        annual_salary: "100000".to_string(), // Amount in cents as string
        account_purpose: "Personal use".to_string(),
        expected_monthly_volume: "5000".to_string(), // Amount in cents as string
        is_terms_of_service_accepted: true,
        sumsub_share_token: "sumsub-token-123".to_string(), // Required: From SumSub integration
        wallet_address: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
        solana_address: None,
        tron_address: None,
        stellar_address: None,
        chain_id: None,
        contract_address: None,
        source_key: Some("my-source-key".to_string()),
        has_existing_documents: None,
    };

    let application = client.create_user_application(&application_request).await?;
    println!("Application created! User ID: {}", application.id);
    println!("Application Status: {:?}", application.application_status);

    let user_id = application.id;

    // Step 2: Upload supporting documents
    println!("\nStep 2: Uploading supporting documents...");

    // Upload ID card front
    let document_params = DocumentUploadParams {
        document_type: "idCard".to_string(),
        side: "front".to_string(),
        country: Some("US".to_string()),
        country_code: Some("US".to_string()),
        name: None,
        file_path: "/path/to/id_front.jpg".to_string(),
    };

    client
        .upload_user_document(&application.id, &document_params)
        .await?;
    println!("ID card front uploaded");

    // Upload ID card back
    let document_params = DocumentUploadParams {
        document_type: "idCard".to_string(),
        side: "back".to_string(),
        country: Some("US".to_string()),
        country_code: Some("US".to_string()),
        name: None,
        file_path: "/path/to/id_back.jpg".to_string(),
    };

    client
        .upload_user_document(&user_id, &document_params)
        .await?;
    println!("ID card back uploaded");

    // Step 3: Get application updates
    println!("\nStep 3: Checking application status...");

    let updated_user = client.get_user(&user_id).await?;
    println!(
        "Current Application Status: {:?}",
        updated_user.application_status
    );

    // Check if additional verification is needed
    if let Some(verification_link) = updated_user.application_external_verification_link {
        println!("\nAdditional verification required!");
        println!("Verification URL: {}", verification_link.url);
        println!("Verification Params: {:?}", verification_link.params);
        println!(
            "Redirect user to: {}?userId={}&redirect=https://yourapp.com/return",
            verification_link.url, verification_link.params.user_id
        );
    }

    // Step 4: Resubmit application if needed (when status requires more information)
    if let Some(status) = updated_user.application_status {
        if matches!(
            status,
            ApplicationStatus::InReview | ApplicationStatus::Pending
        ) {
            println!("\nStep 4: Updating application with additional information...");

            let update_request = UpdateUserApplicationRequest {
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
                birth_date: Some("1990-01-01".to_string()),
                national_id: Some("123456789".to_string()),
                country_of_issue: Some("US".to_string()),
                address: Some(Address {
                    line1: "123 Main St".to_string(),
                    line2: Some("Apt 4B".to_string()),
                    city: "New York".to_string(),
                    region: "NY".to_string(),
                    postal_code: "10001".to_string(),
                    country_code: "US".to_string(),
                    country: None,
                }),
                ip_address: Some("192.168.1.1".to_string()),
                occupation: Some("Software Engineer".to_string()),
                annual_salary: Some("100000".to_string()),
                account_purpose: Some("Personal use".to_string()),
                expected_monthly_volume: Some("5000".to_string()),
                is_terms_of_service_accepted: Some(true),
                has_existing_documents: None,
            };

            let updated_application = client
                .update_user_application(&user_id, &update_request)
                .await?;
            println!(
                "Application updated! New status: {:?}",
                updated_application.application_status
            );
        }
    }

    // Monitor application status
    println!("\nMonitoring application status...");
    loop {
        let user = client.get_user(&user_id).await?;
        let status = user
            .application_status
            .unwrap_or(ApplicationStatus::Pending);
        println!("Current status: {:?}", status);

        match status {
            ApplicationStatus::Approved => {
                println!("✅ Application approved! User can now use the card.");
                break;
            }
            ApplicationStatus::Rejected => {
                println!("❌ Application rejected.");
                break;
            }
            ApplicationStatus::InReview | ApplicationStatus::Pending => {
                // Check if additional verification is needed
                if let Some(link) = user.application_external_verification_link {
                    println!("⏳ Additional verification required.");
                    println!("Verification URL: {}", link.url);
                    println!(
                        "Redirect user to: {}?userId={}&redirect=https://yourapp.com/return",
                        link.url, link.params.user_id
                    );
                    break;
                }
                println!("⏳ Still processing... Waiting 5 seconds before next check.");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }

    Ok(())
}
