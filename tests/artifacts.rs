mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::artifacts::{
    CreateArtifactParams, DeleteArtifactParams, GetArtifactParams, UpdateArtifactParams,
};

use peridio_sdk::api::{Api, ApiOptions};
use serde_json::json;

#[tokio::test]
async fn create_artifact() {
    let mut server = Server::new_async().await;
    let expected_custom_metadata = json!({ "foo": "bar" });
    let expected_description = "test";
    let expected_name = "a";
    let expected_organization_prn = "string";
    let expected_id = "uuid";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", &*format!("/artifacts"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifacts-create-201.json")
        .create_async()
        .await;

    let params = CreateArtifactParams {
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        id: Some(expected_id.to_string()),
        name: expected_name.to_string(),
    };

    match api.artifacts().create(params).await.unwrap() {
        Some(artifact) => {
            assert_eq!(
                artifact.artifact.custom_metadata,
                Some(expected_custom_metadata.as_object().unwrap().clone())
            );
            assert_eq!(
                artifact.artifact.description,
                Some(expected_description.to_string())
            );
            assert_eq!(artifact.artifact.name, expected_name.to_string());
            assert_eq!(
                artifact.artifact.organization_prn,
                expected_organization_prn.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;

    let expected_custom_metadata = json!({ "foo": "a".repeat(1_000_000 ) });

    let m = server
        .mock("POST", &*format!("/artifacts"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifacts-create-201.json")
        .create_async()
        .await;

    let params = CreateArtifactParams {
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        id: Some(expected_id.to_string()),
        name: expected_name.to_string(),
    };

    match api.artifacts().create(params).await {
        Ok(_artifact) => panic!(),
        Err(err) => assert!(err
            .to_string()
            .contains("Validation error: greater than 1MB")),
    }

    m.expect(0);
}

#[tokio::test]
async fn delete_artifact() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("DELETE", &*format!("/artifacts/{expected_prn}"))
        .with_status(204)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let params = DeleteArtifactParams {
        prn: expected_prn.to_string(),
    };

    match api.artifacts().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_artifact() {
    let mut server = Server::new_async().await;
    let expected_description = "test";
    let expected_name = "a";
    let expected_organization_prn = "string";
    let expected_prn = "1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("GET", &*format!("/artifacts/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifacts-get-200.json")
        .create_async()
        .await;

    let params = GetArtifactParams {
        prn: expected_prn.to_string(),
    };

    match api.artifacts().get(params).await.unwrap() {
        Some(artifact) => {
            assert_eq!(
                artifact.artifact.description,
                Some(expected_description.to_string())
            );
            assert_eq!(artifact.artifact.name, expected_name.to_string());
            assert_eq!(
                artifact.artifact.organization_prn,
                expected_organization_prn.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_artifact() {
    let mut server = Server::new_async().await;
    let expected_custom_metadata = json!({ "foo": "bar" });
    let expected_description = "test-update";
    let expected_name = "b";
    let expected_organization_prn = "string";
    let expected_prn = "1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("PATCH", &*format!("/artifacts/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifacts-update-200.json")
        .create_async()
        .await;

    let params = UpdateArtifactParams {
        prn: expected_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        name: Some(expected_name.to_string()),
    };

    match api.artifacts().update(params).await.unwrap() {
        Some(artifact) => {
            assert_eq!(
                artifact.artifact.custom_metadata,
                Some(expected_custom_metadata.as_object().unwrap().clone())
            );
            assert_eq!(
                artifact.artifact.description,
                Some(expected_description.to_string())
            );
            assert_eq!(artifact.artifact.name, expected_name.to_string());
            assert_eq!(
                artifact.artifact.organization_prn,
                expected_organization_prn.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;

    let expected_custom_metadata = json!({ "foo": "a".repeat(1_000_000 ) });

    let m = server
        .mock("PATCH", &*format!("/artifacts/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/artifacts-update-200.json")
        .create_async()
        .await;

    let params = UpdateArtifactParams {
        prn: expected_prn.to_string(),
        custom_metadata: Some(expected_custom_metadata.as_object().unwrap().clone()),
        description: Some(expected_description.to_string()),
        name: Some(expected_name.to_string()),
    };

    match api.artifacts().update(params).await {
        Ok(_artifact) => panic!(),
        Err(err) => assert!(err
            .to_string()
            .contains("Validation error: greater than 1MB")),
    }

    m.expect(0);
}
