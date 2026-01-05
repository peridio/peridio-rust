use peridio_sdk::{Api, ApiOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    println!("=== Peridio SDK Debug Logging Example ===");
    println!("This example demonstrates debug logging for HTTP requests and responses.");
    println!("Run with: RUST_LOG=debug cargo run --example comprehensive_logging");
    println!();

    // Create API client
    let api = Api::new(ApiOptions {
        api_key: std::env::var("PERIDIO_API_KEY")
            .unwrap_or_else(|_| "demo-invalid-key".to_string()),
        endpoint: std::env::var("PERIDIO_ENDPOINT").ok(),
        ca_bundle_path: None,
        api_version: 1,
    });

    println!("🔍 Debug logging will show:");
    println!("  • HTTP method and full URL for each request");
    println!("  • Response status codes");
    println!("  • peridio-request-id headers (for request tracing)");
    println!("  • Response bodies (JSON formatted when possible)");
    println!();

    // Test 1: GET request to /users/me (likely to return 401/403)
    println!("📡 Test 1: GET /users/me");
    match api.users().me().await {
        Ok(user) => {
            if let Some(user_data) = user {
                println!("✅ Success: Retrieved user data");
                println!("   Email: {}", user_data.data.email);
                println!("   Username: {}", user_data.data.username);
            } else {
                println!("✅ Success: Request completed but no data returned (204 No Content)");
            }
        }
        Err(e) => {
            println!("❌ Expected error: {}", e);
            println!("   (This is normal with an invalid API key)");
        }
    }
    println!();

    // Test 2: List products (another common endpoint)
    println!("📡 Test 2: GET /products");
    use peridio_sdk::api::products::ListProductsParams;
    use peridio_sdk::list_params::ListParams;
    let list_params = ListProductsParams {
        list: ListParams::default(),
    };
    match api.products().list(list_params).await {
        Ok(products) => {
            if let Some(products_data) = products {
                println!(
                    "✅ Success: Retrieved {} products",
                    products_data.products.len()
                );
            } else {
                println!("✅ Success: No products found (204 No Content)");
            }
        }
        Err(e) => {
            println!("❌ Expected error: {}", e);
        }
    }
    println!();

    // Test 3: Try to create something (will likely fail with validation error)
    println!("📡 Test 3: POST /products (invalid data to show validation errors)");
    use peridio_sdk::api::products::CreateProductParams;
    let invalid_product = CreateProductParams {
        name: "".to_string(), // Empty name should cause validation error
        archived: None,
    };

    match api.products().create(invalid_product).await {
        Ok(product) => {
            if let Some(_product_data) = product {
                println!("✅ Unexpected success: Created product");
            }
        }
        Err(e) => {
            println!("❌ Expected validation error: {}", e);
            println!("   (This demonstrates how validation errors are logged)");
        }
    }
    println!();

    println!("🎯 Summary:");
    println!("  The debug logs above show the complete HTTP conversation:");
    println!("  • Request URLs help with debugging endpoint issues");
    println!("  • Status codes help identify the type of error");
    println!("  • peridio-request-id headers help correlate requests with server logs");
    println!("  • JSON-formatted responses help understand API error details");
    println!();
    println!("💡 Tip: Use RUST_LOG=peridio_sdk=debug to see only SDK logs");
    println!("   or RUST_LOG=peridio_sdk::api=debug for just API request logs");

    Ok(())
}
