use peridio_sdk::{Api, ApiOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger - set RUST_LOG=debug to see all debug messages
    // or RUST_LOG=peridio_sdk=debug to see only SDK debug messages
    env_logger::init();

    // Create API client - you'll need to set a valid API key
    let api = Api::new(ApiOptions {
        api_key: std::env::var("PERIDIO_API_KEY")
            .unwrap_or_else(|_| "your-api-key-here".to_string()),
        endpoint: None, // Use default endpoint
        ca_bundle_path: None,
        api_version: 1,
    });

    println!("Making API request with debug logging enabled...");
    println!("Set RUST_LOG=debug environment variable to see detailed logging");
    println!("Example: RUST_LOG=debug cargo run --example debug_logging");
    println!();

    // Try to make a request - this will show debug logging
    // Note: This might fail if you don't have a valid API key, but you'll still see the debug logs
    match api.users().me().await {
        Ok(user_response) => {
            if let Some(user) = user_response {
                println!("Successfully retrieved user: {:?}", user);
            } else {
                println!("Request successful but no user data returned");
            }
        }
        Err(e) => {
            println!(
                "Request failed (this is expected with invalid API key): {}",
                e
            );
            println!("Check the debug logs above to see the URL and response details");
        }
    }

    Ok(())
}
