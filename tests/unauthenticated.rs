mod common;

use common::API_KEY;
use mockito::Server;
use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;
use peridio_sdk::api::Error::{HttpError, StructuredError};

#[tokio::test]
async fn bad_params() {
    let mut server = Server::new_async().await;
    let m = server
        .mock("GET", "/users/me")
        .with_status(403)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/403.json")
        .create_async()
        .await;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let result = api.users().me().await;
    assert!(matches!(
        result,
        Err(StructuredError { status: 403, .. }) | Err(HttpError { status: 403, .. })
    ));
    m.assert_async().await;
}

#[tokio::test]
async fn validation_error_422() {
    let mut server = Server::new_async().await;
    let m = server
        .mock("GET", "/users/me")
        .with_status(422)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/422.json")
        .create_async()
        .await;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let result = api.users().me().await;
    assert!(matches!(
        result,
        Err(StructuredError { status: 422, .. }) | Err(HttpError { status: 422, .. })
    ));
    m.assert_async().await;
}
