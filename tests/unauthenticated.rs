mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};
use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;
use peridio_sdk::api::Error::Unknown;

#[tokio::test]
async fn bad_params() {
    let m = mock("GET", "/users/me")
        .with_status(403)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/403.json")
        .create();

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let result = api.users().me().await;
    assert!(matches!(result, Err(Unknown { .. })));
    m.assert();
}
