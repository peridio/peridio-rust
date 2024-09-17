mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::bundles::{CreateBundleParams, GetBundleParams, UpdateBundleParams};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_bundle() {
    let mut server = Server::new_async().await;
    let expected_organization_prn = "organization_prn";
    let expected_artifact_versions = [
        "artifact_version_prn_1".to_string(),
        "artifact_version_prn_2".to_string(),
    ]
    .to_vec();
    let expected_name = "a";
    let expected_id = "uuid";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", "/bundles")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-create-201.json")
        .create_async()
        .await;

    let params = CreateBundleParams {
        organization_prn: expected_organization_prn.to_string(),
        artifact_version_prns: expected_artifact_versions.clone(),
        id: Some(expected_id.to_string()),
        name: Some(expected_name.to_string()),
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

    m.assert_async().await;
}

#[tokio::test]
async fn get_bundle() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";
    let expected_organization_prn = "organization_prn";
    let expected_artifact_versions = [
        "artifact_version_prn_1".to_string(),
        "artifact_version_prn_2".to_string(),
    ]
    .to_vec();

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/bundles/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-get-200.json")
        .create_async()
        .await;

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

    m.assert_async().await;
}

#[tokio::test]
async fn update_bundle() {
    let mut server = Server::new_async().await;
    let expected_name = "b";
    let expected_prn = "1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("PATCH", &*format!("/bundles/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundles-update-200.json")
        .create_async()
        .await;

    let params = UpdateBundleParams {
        prn: expected_prn.to_string(),
        name: Some(expected_name.to_string()),
    };

    match api.bundles().update(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.bundle.name, Some(expected_name.to_string()));
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
