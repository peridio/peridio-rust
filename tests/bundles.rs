mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};

use peridio_sdk::api::bundles::{CreateBundleParams, GetBundleParams};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_bundle() {
    let expected_organization_prn = "organization_prn";
    let expected_artifact_versions = [
        "artifact_version_prn_1".to_string(),
        "artifact_version_prn_2".to_string(),
    ]
    .to_vec();

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("POST", &*format!("/bundles"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-create-201.json")
        .create();

    let params = CreateBundleParams {
        organization_prn: expected_organization_prn.to_string(),
        artifact_version_prns: expected_artifact_versions.clone(),
    };

    match api.bundles().create(params).await.unwrap() {
        Some(bundle) => {
            assert_eq!(
                bundle.bundle.organization_prn,
                expected_organization_prn.to_string()
            );

            assert_eq!(bundle.bundle.artifact_versions, expected_artifact_versions);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn get_bundle() {
    let expected_prn = "prn";
    let expected_organization_prn = "organization_prn";
    let expected_artifact_versions = [
        "artifact_version_prn_1".to_string(),
        "artifact_version_prn_2".to_string(),
    ]
    .to_vec();

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("GET", &*format!("/bundles/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-get-200.json")
        .create();

    let params = GetBundleParams {
        prn: expected_prn.to_string(),
    };

    match api.bundles().get(params).await.unwrap() {
        Some(bundle) => {
            assert_eq!(
                bundle.bundle.organization_prn,
                expected_organization_prn.to_string()
            );

            assert_eq!(bundle.bundle.artifact_versions, expected_artifact_versions);
        }
        _ => panic!(),
    }

    m.assert();
}
