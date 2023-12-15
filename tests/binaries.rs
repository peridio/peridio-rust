mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};

use peridio_sdk::api::binaries::{CreateBinaryParams, GetBinaryParams};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_binary() {
    let expected_artifact_version_prn = "artifact_version_prn";
    let expected_description = "description";
    let expected_hash = "hash";
    let expected_organization_prn = "organization_prn";
    let expected_size = 10;
    let expected_target = "target";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("POST", &*format!("/binaries"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binaries-create-201.json")
        .create();

    let params = CreateBinaryParams {
        artifact_version_prn: expected_artifact_version_prn.to_string(),
        description: Some(expected_description.to_string()),
        hash: expected_hash.to_string(),
        size: expected_size,
        target: expected_target.to_string(),
    };

    match api.binaries().create(params).await.unwrap() {
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

    m.assert();
}

#[tokio::test]
async fn get_binary() {
    let expected_prn = "prn";
    let expected_artifact_version_prn = "artifact_version_prn";
    let expected_description = "description";
    let expected_hash = "hash";
    let expected_organization_prn = "organization_prn";
    let expected_size = 10;
    let expected_target = "target";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("GET", &*format!("/binaries/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/binaries-get-200.json")
        .create();

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

    m.assert();
}
