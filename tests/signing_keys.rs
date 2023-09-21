mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};

use peridio_sdk::api::signing_keys::{
    CreateSigningKeyParams, DeleteSigningKeyParams, GetSigningKeyParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_signing_key() {
    let organization_prn = "org-1";
    let expected_value = "a";
    let expected_name = "b";
    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("POST", &*format!("/signing_keys"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/signing-keys-create-201.json")
        .create();

    let params = CreateSigningKeyParams {
        value: expected_value.to_string(),
        name: expected_name.to_string(),
        organization_prn: organization_prn.to_string(),
    };

    match api.signing_keys().create(params).await.unwrap() {
        Some(signing_key) => {
            assert_eq!(
                signing_key.signing_key.value,
                Some(expected_value.to_string())
            );
            assert_eq!(signing_key.signing_key.name, expected_name);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn delete_signing_key() {
    let signing_key_prn = "b";
    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("DELETE", &*format!("/signing_keys/{signing_key_prn}"))
        .with_status(204)
        .with_body("")
        .create();

    let params = DeleteSigningKeyParams {
        signing_key_prn: signing_key_prn.to_string(),
    };

    match api.signing_keys().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn get_signing_key() {
    let signing_key_prn = "b";
    let expected_value = "a";
    let expected_name = "b";
    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("GET", &*format!("/signing_keys/{signing_key_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/signing-keys-get-200.json")
        .create();

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
        }
        _ => panic!(),
    }

    m.assert();
}
