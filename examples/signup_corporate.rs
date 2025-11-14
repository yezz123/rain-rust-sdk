//! Example: Signing up a corporate customer
//!
//! This example demonstrates the workflow for signing up a corporate customer:
//! 1. Submit an application (creates both company and initial user)
//! 2. Upload supporting documents
//! 3. Get application updates (optional)
//! 4. Resubmit an application if needed (optional)
//! 5. Perform additional verification if needed (optional)

use rain_sdk::models::applications::*;
use rain_sdk::models::common::*;
use rain_sdk::{AuthConfig, Config, Environment, RainClient};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let config = Config::new(Environment::Dev);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = RainClient::new(config, auth)?;

    // Step 1: Submit a corporate application
    // Note: Creating a corporate application also creates a user
    println!("Step 1: Submitting corporate application...");

    let initial_user = InitialUser {
        id: None, // Leave None if this is a new user
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        birth_date: "1985-05-15".to_string(),
        national_id: "987654321".to_string(),
        country_of_issue: "US".to_string(),
        email: "jane.smith@company.com".to_string(),
        ip_address: "192.168.1.100".to_string(),
        is_terms_of_service_accepted: true,
        address: Address {
            line1: "456 Corporate Blvd".to_string(),
            line2: None,
            city: "San Francisco".to_string(),
            region: "CA".to_string(),
            postal_code: "94105".to_string(),
            country_code: "US".to_string(),
            country: None,
        },
        phone_country_code: Some("1".to_string()),
        phone_number: Some("5551234567".to_string()),
        role: Some("CEO".to_string()),
        wallet_address: Some("0xabcdef1234567890abcdef1234567890abcdef12".to_string()),
        solana_address: None,
        tron_address: None,
        stellar_address: None,
    };

    let entity_info = EntityInfo {
        name: "Acme Corporation".to_string(),
        description: "A leading technology company".to_string(),
        industry: "Technology".to_string(),
        registration_number: "CORP-12345".to_string(),
        tax_id: "TAX-98765".to_string(),
        website: "https://acme.com".to_string(),
        r#type: Some("LLC".to_string()),
        expected_spend: Some("50000".to_string()),
    };

    let representative = Representative {
        id: None,
        first_name: "John".to_string(),
        last_name: "Manager".to_string(),
        birth_date: "1980-03-20".to_string(),
        national_id: "111222333".to_string(),
        country_of_issue: "US".to_string(),
        email: "john.manager@company.com".to_string(),
        address: Address {
            line1: "456 Corporate Blvd".to_string(),
            line2: None,
            city: "San Francisco".to_string(),
            region: "CA".to_string(),
            postal_code: "94105".to_string(),
            country_code: "US".to_string(),
            country: None,
        },
        phone_country_code: Some("1".to_string()),
        phone_number: Some("5559876543".to_string()),
    };

    let ubo = UltimateBeneficialOwner {
        id: None,
        first_name: "Alice".to_string(),
        last_name: "Owner".to_string(),
        birth_date: "1975-07-10".to_string(),
        national_id: "444555666".to_string(),
        country_of_issue: "US".to_string(),
        email: "alice.owner@company.com".to_string(),
        address: Address {
            line1: "789 Ownership St".to_string(),
            line2: None,
            city: "Los Angeles".to_string(),
            region: "CA".to_string(),
            postal_code: "90001".to_string(),
            country_code: "US".to_string(),
            country: None,
        },
        phone_country_code: Some("1".to_string()),
        phone_number: Some("5551112222".to_string()),
    };

    let application_request = CreateCompanyApplicationRequest {
        name: "Acme Corporation".to_string(),
        address: Address {
            line1: "456 Corporate Blvd".to_string(),
            line2: None,
            city: "San Francisco".to_string(),
            region: "CA".to_string(),
            postal_code: "94105".to_string(),
            country_code: "US".to_string(),
            country: None,
        },
        entity: entity_info,
        initial_user,
        representatives: vec![representative],
        ultimate_beneficial_owners: vec![ubo],
        chain_id: None,
        contract_address: None,
        source_key: Some("corporate-source-key".to_string()),
    };

    let application = client
        .create_company_application(&application_request)
        .await?;
    println!("Application created! Company ID: {}", application.id);
    println!("Application Status: {:?}", application.application_status);

    // Note: The initial user is created as part of the company application
    // You may need to query for users associated with this company to get the initial user ID
    // For this example, we'll demonstrate document uploads assuming we have the IDs
    println!("Company created. Initial user has been created automatically.");
    println!("To get the initial user ID, you can list users for this company.");

    // Step 2: Upload supporting documents
    println!("\nStep 2: Uploading supporting documents...");

    // Upload company documents
    let company_document_params = DocumentUploadParams {
        document_type: "directorsRegistry".to_string(),
        side: "front".to_string(),
        country: Some("US".to_string()),
        country_code: Some("US".to_string()),
        name: Some("Directors Registry".to_string()),
        file_path: "/path/to/directors_registry.pdf".to_string(),
    };

    client
        .upload_company_document(&application.id, &company_document_params)
        .await?;
    println!("Company document uploaded");

    // Upload UBO documents
    // UBO IDs are in the application response
    // Note: Representatives don't have separate document upload endpoints
    // They are part of the company application
    let ubo_id = application
        .ultimate_beneficial_owners
        .as_ref()
        .and_then(|ubos| ubos.first())
        .map(|u| u.id)
        .unwrap_or_else(|| {
            println!("Note: UBO ID needed - check application response for actual IDs");
            Uuid::new_v4() // Placeholder - replace with actual UBO ID
        });

    let ubo_document_params = DocumentUploadParams {
        document_type: "idCard".to_string(),
        side: "front".to_string(),
        country: Some("US".to_string()),
        country_code: Some("US".to_string()),
        name: None,
        file_path: "/path/to/ubo_id_front.jpg".to_string(),
    };

    client
        .upload_ubo_document(&application.id, &ubo_id, &ubo_document_params)
        .await?;
    println!("UBO document uploaded");

    // Step 3: Get application updates
    println!("\nStep 3: Checking application status...");

    let updated_company = client.get_company(&application.id).await?;
    println!(
        "Current Application Status: {:?}",
        updated_company.application_status
    );

    // Check if additional verification is needed
    if let Some(verification_link) = updated_company.application_external_verification_link {
        println!("\nAdditional verification required!");
        println!("Verification URL: {}", verification_link.url);
        println!("Verification Params: {:?}", verification_link.params);
    }

    // Step 4: Resubmit application if needed
    if let Some(status) = updated_company.application_status {
        if matches!(
            status,
            ApplicationStatus::InReview | ApplicationStatus::Pending
        ) {
            println!("\nStep 4: Updating application with additional information...");

            let update_request = UpdateCompanyApplicationRequest {
                name: None,
                address: Some(Address {
                    line1: "456 Corporate Blvd".to_string(),
                    line2: Some("Suite 200".to_string()),
                    city: "San Francisco".to_string(),
                    region: "CA".to_string(),
                    postal_code: "94105".to_string(),
                    country_code: "US".to_string(),
                    country: None,
                }),
                entity: None,
            };

            let updated_application = client
                .update_company_application(&application.id, &update_request)
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
        let company = client.get_company(&application.id).await?;
        let status = company
            .application_status
            .unwrap_or(ApplicationStatus::Pending);
        println!("Current status: {:?}", status);

        match status {
            ApplicationStatus::Approved => {
                println!("✅ Application approved! Company can now use cards.");
                break;
            }
            ApplicationStatus::Rejected => {
                println!("❌ Application rejected.");
                break;
            }
            ApplicationStatus::InReview | ApplicationStatus::Pending => {
                // Check if additional verification is needed
                if let Some(link) = company.application_external_verification_link {
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
