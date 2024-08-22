mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::binaries::{
    BinaryState, CreateBinaryParams, GetBinaryParams, UpdateBinaryParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;
use serde_json::json;

#[tokio::test]
async fn create_binary() {
    let mut server = Server::new_async().await;
    let expected_artifact_version_prn = "artifact_version_prn";
    let expected_custom_metadata = json!({ "foo": "bar" });
    let expected_description = "description";
    let expected_hash = "hash";
    let expected_organization_prn = "organization_prn";
    let expected_size = 10;
    let expected_target = "target";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/binaries"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binaries-create-201.json")
        .create_async()
        .await;

    let params = CreateBinaryParams {
        artifact_version_prn: expected_artifact_version_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        hash: expected_hash.to_string(),
        size: expected_size,
        target: expected_target.to_string(),
    };

    match api.binaries().create(params).await.unwrap() {
        Some(binary) => {
            assert_eq!(
                binary.binary.custom_metadata,
                Some(expected_custom_metadata.as_object().unwrap().clone())
            );
            assert_eq!(
                binary.binary.artifact_version_prn,
                expected_artifact_version_prn.to_string()
            );
            assert_eq!(
                binary.binary.description,
                Some(expected_description.to_string())
            );
            assert_eq!(binary.binary.hash, Some(expected_hash.to_string()));
            assert_eq!(
                binary.binary.organization_prn,
                expected_organization_prn.to_string()
            );
            assert_eq!(binary.binary.size, Some(expected_size));
            assert_eq!(binary.binary.target, expected_target.to_string());
        }
        _ => panic!(),
    }

    m.assert_async().await;

    let expected_custom_metadata = json!({ "foo": "a".repeat(1_000_000 ) });

    let m = server
        .mock("POST", &*format!("/binaries"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binaries-create-201.json")
        .create_async()
        .await;

    let params = CreateBinaryParams {
        artifact_version_prn: expected_artifact_version_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        hash: expected_hash.to_string(),
        size: expected_size,
        target: expected_target.to_string(),
    };

    match api.binaries().create(params).await {
        Ok(_binary) => panic!(),
        Err(err) => assert!(err
            .to_string()
            .contains("Validation error: greater than 1MB")),
    }

    m.expect(0);
}

#[tokio::test]
async fn get_binary() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";
    let expected_artifact_version_prn = "artifact_version_prn";
    let expected_description = "description";
    let expected_hash = "hash";
    let expected_organization_prn = "organization_prn";
    let expected_size = 10;
    let expected_target = "target";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/binaries/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binaries-get-200.json")
        .create_async()
        .await;

    let params = GetBinaryParams {
        prn: expected_prn.to_string(),
    };

    match api.binaries().get(params).await.unwrap() {
        Some(binary) => {
            assert_eq!(
                binary.binary.artifact_version_prn,
                expected_artifact_version_prn.to_string()
            );
            assert_eq!(
                binary.binary.description,
                Some(expected_description.to_string())
            );
            assert_eq!(binary.binary.hash, Some(expected_hash.to_string()));
            assert_eq!(
                binary.binary.organization_prn,
                expected_organization_prn.to_string()
            );
            assert_eq!(binary.binary.size, Some(expected_size));
            assert_eq!(binary.binary.target, expected_target.to_string());
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_binary() {
    let mut server = Server::new_async().await;
    let expected_artifact_version_prn = "artifact_version_prn";
    let expected_custom_metadata = json!({ "foo": "bar" });
    let expected_description = "description";
    let expected_hash = "hash";
    let expected_organization_prn = "organization_prn";
    let expected_size = 10;
    let expected_prn = "1";
    let expected_state = BinaryState::Signed;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("PATCH", &*format!("/binaries/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binaries-update-200.json")
        .create_async()
        .await;

    let params = UpdateBinaryParams {
        prn: expected_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        hash: Some(expected_hash.to_string()),
        size: Some(expected_size),
        state: Some(expected_state.clone()),
    };

    match api.binaries().update(params).await.unwrap() {
        Some(binary) => {
            assert_eq!(
                binary.binary.custom_metadata,
                Some(expected_custom_metadata.as_object().unwrap().clone())
            );
            assert_eq!(
                binary.binary.artifact_version_prn,
                expected_artifact_version_prn.to_string()
            );
            assert_eq!(
                binary.binary.description,
                Some(expected_description.to_string())
            );
            assert_eq!(binary.binary.hash, Some(expected_hash.to_string()));
            assert_eq!(
                binary.binary.organization_prn,
                expected_organization_prn.to_string()
            );
            assert_eq!(binary.binary.size, Some(expected_size));
            assert!(matches!(binary.binary.state, BinaryState::Signed));
        }
        _ => panic!(),
    }

    m.assert_async().await;

    let expected_custom_metadata = json!({ "foo": "a".repeat(1_000_000 ) });

    let m = server
        .mock("PATCH", &*format!("/binaries/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binaries-update-200.json")
        .create_async()
        .await;

    let params = UpdateBinaryParams {
        prn: expected_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        hash: Some(expected_hash.to_string()),
        size: Some(expected_size),
        state: Some(expected_state),
    };

    match api.binaries().update(params).await {
        Ok(_binary) => panic!(),
        Err(err) => assert!(err
            .to_string()
            .contains("Validation error: greater than 1MB")),
    }

    m.expect(0);
}
