use peridio_sdk::{Api, ApiOptions};

#[tokio::test]
async fn test_logging_does_not_affect_functionality() {
    // Initialize logger for testing
    let _ = env_logger::builder().is_test(true).try_init();

    // Create API client with invalid key to get predictable error
    let api = Api::new(ApiOptions {
        api_key: "test-invalid-key".to_string(),
        endpoint: None,
        ca_bundle_path: None,
        api_version: 1,
    });

    // Make a request that should fail with 403
    let result = api.users().me().await;

    // Verify we get the expected error (not affected by logging)
    match result {
        Err(peridio_sdk::api::Error::StructuredError { status, error }) => {
            assert_eq!(status, 403);
            // Verify the error structure is preserved
            match error {
                peridio_sdk::api::error::ApiError::Status { status } => {
                    assert_eq!(status, "forbidden");
                }
                _ => panic!("Expected Status error variant"),
            }
        }
        Err(peridio_sdk::api::Error::HttpError { status, .. }) => {
            // Also acceptable - raw HTTP error
            assert_eq!(status, 403);
        }
        other => panic!("Expected 403 error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_logging_with_multiple_requests() {
    // Initialize logger for testing
    let _ = env_logger::builder().is_test(true).try_init();

    let api = Api::new(ApiOptions {
        api_key: "test-key-multiple".to_string(),
        endpoint: None,
        ca_bundle_path: None,
        api_version: 1,
    });

    // Make multiple requests to ensure logging doesn't interfere
    let results =
        futures::future::join_all(vec![api.users().me(), api.users().me(), api.users().me()]).await;

    // All should fail with 403, regardless of logging
    for result in results {
        match result {
            Err(peridio_sdk::api::Error::StructuredError { status, .. })
            | Err(peridio_sdk::api::Error::HttpError { status, .. }) => {
                assert_eq!(status, 403);
            }
            other => panic!("Expected 403 error, got: {:?}", other),
        }
    }
}

#[test]
fn test_debug_log_format() {
    // Test that our log formatting doesn't panic with edge cases
    use log::debug;

    // Initialize a test logger
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    // Test various JSON formatting scenarios
    let valid_json = r#"{"status": "ok", "data": {"key": "value"}}"#;
    let invalid_json = "not json at all";
    let empty_string = "";

    // These should not panic
    debug!("Test JSON formatting: {}", valid_json);
    debug!("Test non-JSON: {}", invalid_json);
    debug!("Test empty: {}", empty_string);

    // Test pretty printing
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(valid_json) {
        let pretty = serde_json::to_string_pretty(&json_value).unwrap();
        debug!("Test pretty JSON: {}", pretty);
    }
}

#[test]
fn test_peridio_request_id_header_logging() {
    // Test that we properly log peridio-request-id headers when present
    use log::debug;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // Capture log messages
    struct TestLogger {
        messages: Arc<Mutex<Vec<String>>>,
    }

    impl log::Log for TestLogger {
        fn enabled(&self, _metadata: &log::Metadata) -> bool {
            true
        }

        fn log(&self, record: &log::Record) {
            if record.level() == log::Level::Debug {
                let mut messages = self.messages.lock().unwrap();
                messages.push(format!("{}", record.args()));
            }
        }

        fn flush(&self) {}
    }

    let messages = Arc::new(Mutex::new(Vec::new()));
    let logger = TestLogger {
        messages: messages.clone(),
    };

    // Set up the test logger
    let _ = log::set_boxed_logger(Box::new(logger))
        .map(|()| log::set_max_level(log::LevelFilter::Debug));

    // Simulate what happens in execute_full when we log headers
    let mut headers = HashMap::new();
    headers.insert("peridio-request-id", "test-request-id-12345");
    headers.insert("content-type", "application/json");

    // Test logging a request ID header
    if let Some(request_id) = headers.get("peridio-request-id") {
        debug!("peridio-request-id: {}", request_id);
    }

    // Verify the request ID was logged
    let captured_messages = messages.lock().unwrap();
    let found_request_id = captured_messages
        .iter()
        .any(|msg| msg.contains("peridio-request-id: test-request-id-12345"));

    assert!(
        found_request_id,
        "Expected to find peridio-request-id in logs: {:?}",
        *captured_messages
    );
}
