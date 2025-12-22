mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::signing_keys::{
    CreateSigningKeyParams, DeleteSigningKeyParams, GetSigningKeyParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_signing_key() {
    let mut server = Server::new_async().await;
    let expected_value = "a";
    let expected_name = "b";
    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", &*format!("/signing_keys"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/signing-keys-create-201.json")
        .create_async()
        .await;

    let params = CreateSigningKeyParams {
        value: expected_value.to_string(),
        name: expected_name.to_string(),
    };

    match api.signing_keys().create(params).await.unwrap() {
        Some(signing_key) => {
            assert_eq!(
                signing_key.signing_key.value,
                Some(expected_value.to_string())
            );
            assert_eq!(signing_key.signing_key.name, expected_name);
            assert_eq!(signing_key.signing_key.keyid, "test-keyid-123");
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_signing_key() {
    let mut server = Server::new_async().await;
    let signing_key_prn = "b";
    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("DELETE", &*format!("/signing_keys/{signing_key_prn}"))
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = DeleteSigningKeyParams {
        signing_key_prn: signing_key_prn.to_string(),
    };

    match api.signing_keys().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_signing_key() {
    let mut server = Server::new_async().await;
    let signing_key_prn = "b";
    let expected_value = "a";
    let expected_name = "b";
    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("GET", &*format!("/signing_keys/{signing_key_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/signing-keys-get-200.json")
        .create_async()
        .await;

    let params = GetSigningKeyParams {
        prn: signing_key_prn.to_string(),
    };

    match api.signing_keys().get(params).await.unwrap() {
        Some(signing_key) => {
            assert_eq!(
                signing_key.signing_key.value,
                Some(expected_value.to_string())
            );
            assert_eq!(signing_key.signing_key.name, expected_name);
            assert_eq!(signing_key.signing_key.keyid, "test-keyid-456");
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
