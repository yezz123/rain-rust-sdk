//! Initiate user application example

use rain_sdk::models::applications::InitiateUserApplicationRequest;
use rain_sdk::{AuthConfig, Config, Environment, RainClient};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration for dev environment
    let config = Config::new(Environment::Dev);

    // Create authentication config with API key
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());

    // Create the client
    let client = RainClient::new(config, auth)?;

    // Initiate a user application
    let request = InitiateUserApplicationRequest {
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("john.doe@example.com".to_string()),
        wallet_address: Some("0x1234567890123456789012345678901234567890".to_string()),
    };

    let application = client.initiate_user_application(&request).await?;

    println!("User application created successfully!");
    println!("Application ID: {}", application.id);
    println!("Email: {:?}", application.email);
    println!("Application Status: {:?}", application.application_status);

    Ok(())
}

#[cfg(feature = "sync")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration for dev environment
    let config = Config::new(Environment::Dev);

    // Create authentication config with API key
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());

    // Create the client
    let client = RainClient::new(config, auth)?;

    // Initiate a user application
    let request = InitiateUserApplicationRequest {
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("john.doe@example.com".to_string()),
        wallet_address: Some("0x1234567890123456789012345678901234567890".to_string()),
    };

    let application = client.initiate_user_application_blocking(&request)?;

    println!("User application created successfully!");
    println!("Application ID: {}", application.id);
    println!("Email: {:?}", application.email);
    println!("Application Status: {:?}", application.application_status);

    Ok(())
}
