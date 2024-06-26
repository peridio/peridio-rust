mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};

use peridio_sdk::api::artifact_versions::{
    CreateArtifactVersionParams, GetArtifactVersionParams, UpdateArtifactVersionParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;
use serde_json::json;

#[tokio::test]
async fn create_artifact_version() {
    let expected_artifact_prn = "artifact_prn";
    let expected_custom_metadata = json!({ "foo": "bar" });
    let expected_description = "description";
    let expected_version = "v0.0.1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("POST", &*format!("/artifact_versions"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-create-201.json")
        .create();

    let params = CreateArtifactVersionParams {
        artifact_prn: expected_artifact_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        version: expected_version.to_string(),
    };

    match api.artifact_versions().create(params).await.unwrap() {
        Some(artifact_version) => {
            assert_eq!(
                artifact_version.artifact_version.custom_metadata,
                Some(expected_custom_metadata.as_object().unwrap().clone())
            );
            assert_eq!(
                artifact_version.artifact_version.description,
                Some(expected_description.to_string())
            );
            assert_eq!(
                artifact_version.artifact_version.artifact_prn,
                expected_artifact_prn.to_string()
            );
            assert_eq!(
                artifact_version.artifact_version.version,
                expected_version.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert();

    let expected_custom_metadata = json!({ "foo": "a".repeat(1_048_576 ) });

    let m = mock("POST", &*format!("/artifact_versions"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-create-201.json")
        .create();

    let params = CreateArtifactVersionParams {
        artifact_prn: expected_artifact_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        version: expected_version.to_string(),
    };

    match api.artifact_versions().create(params).await {
        Ok(_artifact_version) => panic!(),
        Err(err) => assert!(err
            .to_string()
            .contains("Validation error: greater than 1MB")),
    }

    m.expect(0);
}

#[tokio::test]
async fn get_artifact_version() {
    let expected_prn = "prn";
    let expected_artifact_prn = "artifact_prn";
    let expected_description = "description";
    let expected_version = "v0.0.1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("GET", &*format!("/artifact_versions/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-get-200.json")
        .create();

    let params = GetArtifactVersionParams {
        prn: expected_prn.to_string(),
    };

    match api.artifact_versions().get(params).await.unwrap() {
        Some(artifact_version) => {
            assert_eq!(
                artifact_version.artifact_version.description,
                Some(expected_description.to_string())
            );
            assert_eq!(
                artifact_version.artifact_version.artifact_prn,
                expected_artifact_prn.to_string()
            );
            assert_eq!(
                artifact_version.artifact_version.version,
                expected_version.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn update_artifact() {
    let expected_prn = "prn";
    let expected_custom_metadata = json!({ "foo": "bar" });
    let expected_description = "updated_description";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("PATCH", &*format!("/artifact_versions/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-update-200.json")
        .create();

    let params = UpdateArtifactVersionParams {
        prn: expected_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
    };

    match api.artifact_versions().update(params).await.unwrap() {
        Some(artifact_version) => {
            assert_eq!(
                artifact_version.artifact_version.custom_metadata,
                Some(expected_custom_metadata.as_object().unwrap().clone())
            );
            assert_eq!(
                artifact_version.artifact_version.description,
                Some(expected_description.to_string())
            )
        }
        _ => panic!(),
    }

    m.assert();

    let expected_custom_metadata = json!({ "foo": "a".repeat(1_048_576 ) });

    let m = mock("PATCH", &*format!("/artifact_versions/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-update-200.json")
        .create();

    let params = UpdateArtifactVersionParams {
        prn: expected_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
    };

    match api.artifact_versions().update(params).await {
        Ok(_artifact_version) => panic!(),
        Err(err) => assert!(err
            .to_string()
            .contains("Validation error: greater than 1MB")),
    }

    m.expect(0);
}
