mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::artifact_versions::{
    CreateArtifactVersionParams, DeleteArtifactVersionParams, GetArtifactVersionParams,
    UpdateArtifactVersionParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;
use serde_json::json;

#[tokio::test]
async fn create_artifact_version() {
    let mut server = Server::new_async().await;
    let expected_artifact_prn = "artifact_prn";
    let expected_custom_metadata = json!({ "foo": "bar" });
    let expected_description = "description";
    let expected_version = "v0.0.1";
    let expected_id = "uuid";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/artifact_versions"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-create-201.json")
        .create_async()
        .await;

    let params = CreateArtifactVersionParams {
        artifact_prn: expected_artifact_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        id: Some(expected_id.to_string()),
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

    m.assert_async().await;

    let expected_custom_metadata = json!({ "foo": "a".repeat(1_000_000 ) });

    let m = server
        .mock("POST", &*format!("/artifact_versions"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-create-201.json")
        .create_async()
        .await;

    let params = CreateArtifactVersionParams {
        artifact_prn: expected_artifact_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        id: Some(expected_id.to_string()),
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
async fn delete_artifact_version() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("DELETE", &*format!("/artifact_versions/{expected_prn}"))
        .with_status(204)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let params = DeleteArtifactVersionParams {
        prn: expected_prn.to_string(),
    };

    match api.artifact_versions().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_artifact_version() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";
    let expected_artifact_prn = "artifact_prn";
    let expected_description = "description";
    let expected_version = "v0.0.1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/artifact_versions/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-get-200.json")
        .create_async()
        .await;

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

    m.assert_async().await;
}

#[tokio::test]
async fn update_artifact() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";
    let expected_custom_metadata = json!({ "foo": "bar" });
    let expected_description = "updated_description";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("PATCH", &*format!("/artifact_versions/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-update-200.json")
        .create_async()
        .await;

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

    m.assert_async().await;

    let expected_custom_metadata = json!({ "foo": "a".repeat(1_000_000 ) });

    let m = server
        .mock("PATCH", &*format!("/artifact_versions/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifact-versions-update-200.json")
        .create_async()
        .await;

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
