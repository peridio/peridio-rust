mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::binary_signatures::{
    CreateBinarySignatureParams, DeleteBinarySignatureParams, ListBinarySignaturesParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;
use peridio_sdk::list_params::ListParams;

#[tokio::test]
async fn create_binary_signature_with_signing_key_prn() {
    let mut server = Server::new_async().await;
    let expected_binary_prn = "prn:1:o:abcd:b:binary-123";
    let expected_signing_key_prn = "prn:1:o:abcd:sk:signing-key-123";
    let expected_signature = "signature-abc123";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/binary_signatures")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binary-signatures-create-201.json")
        .create_async()
        .await;

    let params = CreateBinarySignatureParams {
        binary_prn: expected_binary_prn.to_string(),
        signing_key_prn: Some(expected_signing_key_prn.to_string()),
        signature: expected_signature.to_string(),
        signing_key_keyid: None,
    };

    match api.binary_signatures().create(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.binary_signature.binary_prn, expected_binary_prn);
            assert_eq!(
                response.binary_signature.signing_key_prn,
                expected_signing_key_prn
            );
            assert_eq!(response.binary_signature.signature, expected_signature);
        }
        _ => panic!("Expected binary signature response"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_binary_signature_with_signing_key_keyid() {
    let mut server = Server::new_async().await;
    let expected_binary_prn = "prn:1:o:abcd:b:binary-456";
    let expected_signing_key_keyid = "keyid-xyz789";
    let expected_signature = "signature-xyz789";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/binary_signatures")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binary-signatures-create-keyid-201.json")
        .create_async()
        .await;

    let params = CreateBinarySignatureParams {
        binary_prn: expected_binary_prn.to_string(),
        signing_key_prn: None,
        signature: expected_signature.to_string(),
        signing_key_keyid: Some(expected_signing_key_keyid.to_string()),
    };

    match api.binary_signatures().create(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.binary_signature.binary_prn, expected_binary_prn);
            assert_eq!(response.binary_signature.signature, expected_signature);
        }
        _ => panic!("Expected binary signature response"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_binary_signature_with_both_identifiers() {
    let mut server = Server::new_async().await;
    let expected_binary_prn = "prn:1:o:abcd:b:binary-789";
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
        .mock("POST", "/binary_signatures")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binary-signatures-create-both-201.json")
        .create_async()
        .await;

    let params = CreateBinarySignatureParams {
        binary_prn: expected_binary_prn.to_string(),
        signing_key_prn: Some(expected_signing_key_prn.to_string()),
        signature: expected_signature.to_string(),
        signing_key_keyid: Some(expected_signing_key_keyid.to_string()),
    };

    match api.binary_signatures().create(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.binary_signature.binary_prn, expected_binary_prn);
            assert_eq!(
                response.binary_signature.signing_key_prn,
                expected_signing_key_prn
            );
            assert_eq!(response.binary_signature.signature, expected_signature);
        }
        _ => panic!("Expected binary signature response"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_binary_signature_with_neither_identifier() {
    let mut server = Server::new_async().await;
    let expected_binary_prn = "prn:1:o:abcd:b:binary-none";
    let expected_signature = "signature-none123";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/binary_signatures")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binary-signatures-create-neither-201.json")
        .create_async()
        .await;

    let params = CreateBinarySignatureParams {
        binary_prn: expected_binary_prn.to_string(),
        signing_key_prn: None,
        signature: expected_signature.to_string(),
        signing_key_keyid: None,
    };

    match api.binary_signatures().create(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.binary_signature.binary_prn, expected_binary_prn);
            assert_eq!(response.binary_signature.signature, expected_signature);
        }
        _ => panic!("Expected binary signature response"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_binary_signature() {
    let mut server = Server::new_async().await;
    let binary_signature_prn = "prn:1:o:abcd:bs:binary-signature-123";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock(
            "DELETE",
            format!("/binary_signatures/{}", binary_signature_prn).as_str(),
        )
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = DeleteBinarySignatureParams {
        binary_signature_prn: binary_signature_prn.to_string(),
    };

    match api.binary_signatures().delete(params).await.unwrap() {
        None => (),
        _ => panic!("Expected no response body for delete"),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_binary_signature_serialization_excludes_none_values() {
    // This test verifies that None values are properly excluded from serialization
    use serde_json;

    let params_with_prn_only = CreateBinarySignatureParams {
        binary_prn: "test-binary".to_string(),
        signing_key_prn: Some("test-signing-key".to_string()),
        signature: "test-signature".to_string(),
        signing_key_keyid: None,
    };

    let json = serde_json::to_string(&params_with_prn_only).unwrap();
    assert!(json.contains("signing_key_prn"));
    assert!(!json.contains("signing_key_keyid"));

    let params_with_keyid_only = CreateBinarySignatureParams {
        binary_prn: "test-binary".to_string(),
        signing_key_prn: None,
        signature: "test-signature".to_string(),
        signing_key_keyid: Some("test-keyid".to_string()),
    };

    let json = serde_json::to_string(&params_with_keyid_only).unwrap();
    assert!(!json.contains("signing_key_prn"));
    assert!(json.contains("signing_key_keyid"));

    let params_with_both = CreateBinarySignatureParams {
        binary_prn: "test-binary".to_string(),
        signing_key_prn: Some("test-signing-key".to_string()),
        signature: "test-signature".to_string(),
        signing_key_keyid: Some("test-keyid".to_string()),
    };

    let json = serde_json::to_string(&params_with_both).unwrap();
    assert!(json.contains("signing_key_prn"));
    assert!(json.contains("signing_key_keyid"));

    let params_with_neither = CreateBinarySignatureParams {
        binary_prn: "test-binary".to_string(),
        signing_key_prn: None,
        signature: "test-signature".to_string(),
        signing_key_keyid: None,
    };

    let json = serde_json::to_string(&params_with_neither).unwrap();
    assert!(!json.contains("signing_key_prn"));
    assert!(!json.contains("signing_key_keyid"));
}

#[tokio::test]
async fn list_binary_signatures() {
    let mut server = Server::new_async().await;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("GET", "/binary_signatures")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binary-signatures-list-200.json")
        .create_async()
        .await;

    let params = ListBinarySignaturesParams {
        list: ListParams::default(),
    };

    match api.binary_signatures().list(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.binary_signatures.len(), 2);
            assert_eq!(
                response.binary_signatures[0].binary_prn,
                "prn:1:o:abcd:b:binary-123"
            );
            assert_eq!(
                response.binary_signatures[1].binary_prn,
                "prn:1:o:abcd:b:binary-456"
            );
            assert_eq!(response.next_page, Some("next-page-token".to_string()));
        }
        _ => panic!("Expected binary signatures list response"),
    }

    m.assert_async().await;
}
