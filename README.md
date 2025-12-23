# Peridio Rust SDK

Peridio Rust SDK for [Peridio Admin API](https://docs.peridio.com/admin-api) and [Peridio Device API](https://docs.peridio.com/device-api).

## Debug Logging

The SDK includes comprehensive debug logging that shows:
- HTTP request URLs and methods
- Response status codes
- peridio-request-id headers (for request tracing)
- Response bodies (formatted as JSON when possible)

To enable debug logging, set the `RUST_LOG` environment variable:

```bash
# Enable all debug logging
RUST_LOG=debug cargo run

# Enable only SDK debug logging
RUST_LOG=peridio_sdk=debug cargo run

# Enable only specific modules
RUST_LOG=peridio_sdk::api=debug cargo run
```

### Example Usage

```rust
use peridio_sdk::{Api, ApiOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    let api = Api::new(ApiOptions {
        api_key: "your-api-key".to_string(),
        endpoint: None,
        ca_bundle_path: None,
        api_version: 1,
    });

    // This will log the request URL and response details
    let result = api.users().me().await;
    
    Ok(())
}
```

### Debug Log Output Example

```
[2024-01-15T10:30:45Z DEBUG peridio_sdk::api] Making GET request to URL: https://api.cremini.peridio.com/users/me
[2024-01-15T10:30:46Z DEBUG peridio_sdk::api] Response status code: 200
[2024-01-15T10:30:46Z DEBUG peridio_sdk::api] peridio-request-id: GIPt2hRH5sG1iC4bH3Kh
[2024-01-15T10:30:46Z DEBUG peridio_sdk::api] Response body (JSON): {
  "data": {
    "email": "user@example.com",
    "username": "user",
    "organization_prn": "prn:1:o:org-id"
  }
}
```

The `peridio-request-id` header is particularly useful for:
- Correlating client requests with server logs
- Debugging issues with Peridio support
- Tracking request flows in distributed systems
