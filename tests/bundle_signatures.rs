mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::bundle_signatures::{
    CreateBundleSignatureParams, DeleteBundleSignatureParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_bundle_signature_with_signing_key_prn() {
    let mut server = Server::new_async().await;
    let expected_bundle_prn = "prn:1:o:abcd:b:bundle-123";
    let expected_signing_key_prn = "prn:1:o:abcd:sk:signing-key-123";
    let expected_signature = "signature-abc123";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/bundle_signatures")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-signatures-create-201.json")
        .create_async()
        .await;

    let params = CreateBundleSignatureParams {
        bundle_prn: expected_bundle_prn.to_string(),
        signing_key_prn: Some(expected_signing_key_prn.to_string()),
        signature: expected_signature.to_string(),
        signing_key_keyid: None,
    };

    match api.bundle_signatures().create(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.bundle_signature.bundle_prn, expected_bundle_prn);
            assert_eq!(
                response.bundle_signature.signing_key_prn,
                expected_signing_key_prn
            );
            assert_eq!(response.bundle_signature.signature, expected_signature);
        }
        _ => panic!("Expected bundle signature response"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_bundle_signature_with_signing_key_keyid() {
    let mut server = Server::new_async().await;
    let expected_bundle_prn = "prn:1:o:abcd:b:bundle-456";
    let expected_signing_key_keyid = "keyid-xyz789";
    let expected_signature = "signature-xyz789";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/bundle_signatures")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-signatures-create-keyid-201.json")
        .create_async()
        .await;

    let params = CreateBundleSignatureParams {
        bundle_prn: expected_bundle_prn.to_string(),
        signing_key_prn: None,
        signature: expected_signature.to_string(),
        signing_key_keyid: Some(expected_signing_key_keyid.to_string()),
    };

    match api.bundle_signatures().create(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.bundle_signature.bundle_prn, expected_bundle_prn);
            assert_eq!(response.bundle_signature.signature, expected_signature);
        }
        _ => panic!("Expected bundle signature response"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_bundle_signature_with_both_identifiers() {
    let mut server = Server::new_async().await;
    let expected_bundle_prn = "prn:1:o:abcd:b:bundle-789";
    let expected_signing_key_prn = "prn:1:o:abcd:sk:signing-key-789";
    let expected_signing_key_keyid = "keyid-both123";
    let expected_signature = "signature-both123";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/bundle_signatures")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-signatures-create-both-201.json")
        .create_async()
        .await;

    let params = CreateBundleSignatureParams {
        bundle_prn: expected_bundle_prn.to_string(),
        signing_key_prn: Some(expected_signing_key_prn.to_string()),
        signature: expected_signature.to_string(),
        signing_key_keyid: Some(expected_signing_key_keyid.to_string()),
    };

    match api.bundle_signatures().create(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.bundle_signature.bundle_prn, expected_bundle_prn);
            assert_eq!(
                response.bundle_signature.signing_key_prn,
                expected_signing_key_prn
            );
            assert_eq!(response.bundle_signature.signature, expected_signature);
        }
        _ => panic!("Expected bundle signature response"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_bundle_signature_with_neither_identifier() {
    let mut server = Server::new_async().await;
    let expected_bundle_prn = "prn:1:o:abcd:b:bundle-none";
    let expected_signature = "signature-none123";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/bundle_signatures")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-signatures-create-neither-201.json")
        .create_async()
        .await;

    let params = CreateBundleSignatureParams {
        bundle_prn: expected_bundle_prn.to_string(),
        signing_key_prn: None,
        signature: expected_signature.to_string(),
        signing_key_keyid: None,
    };

    match api.bundle_signatures().create(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.bundle_signature.bundle_prn, expected_bundle_prn);
            assert_eq!(response.bundle_signature.signature, expected_signature);
        }
        _ => panic!("Expected bundle signature response"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_bundle_signature() {
    let mut server = Server::new_async().await;
    let bundle_signature_prn = "prn:1:o:abcd:bs:bundle-signature-123";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock(
            "DELETE",
            format!("/bundle_signatures/{}", bundle_signature_prn).as_str(),
        )
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = DeleteBundleSignatureParams {
        bundle_signature_prn: bundle_signature_prn.to_string(),
    };

    match api.bundle_signatures().delete(params).await.unwrap() {
        None => (),
        _ => panic!("Expected no response body for delete"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_bundle_signature_serialization_excludes_none_values() {
    // This test verifies that None values are properly excluded from serialization
    use serde_json;

    let params_with_prn_only = CreateBundleSignatureParams {
        bundle_prn: "test-bundle".to_string(),
        signing_key_prn: Some("test-signing-key".to_string()),
        signature: "test-signature".to_string(),
        signing_key_keyid: None,
    };

    let json = serde_json::to_string(&params_with_prn_only).unwrap();
    assert!(json.contains("signing_key_prn"));
    assert!(!json.contains("signing_key_keyid"));

    let params_with_keyid_only = CreateBundleSignatureParams {
        bundle_prn: "test-bundle".to_string(),
        signing_key_prn: None,
        signature: "test-signature".to_string(),
        signing_key_keyid: Some("test-keyid".to_string()),
    };

    let json = serde_json::to_string(&params_with_keyid_only).unwrap();
    assert!(!json.contains("signing_key_prn"));
    assert!(json.contains("signing_key_keyid"));

    let params_with_both = CreateBundleSignatureParams {
        bundle_prn: "test-bundle".to_string(),
        signing_key_prn: Some("test-signing-key".to_string()),
        signature: "test-signature".to_string(),
        signing_key_keyid: Some("test-keyid".to_string()),
    };

    let json = serde_json::to_string(&params_with_both).unwrap();
    assert!(json.contains("signing_key_prn"));
    assert!(json.contains("signing_key_keyid"));

    let params_with_neither = CreateBundleSignatureParams {
        bundle_prn: "test-bundle".to_string(),
        signing_key_prn: None,
        signature: "test-signature".to_string(),
        signing_key_keyid: None,
    };

    let json = serde_json::to_string(&params_with_neither).unwrap();
    assert!(!json.contains("signing_key_prn"));
    assert!(!json.contains("signing_key_keyid"));
}
