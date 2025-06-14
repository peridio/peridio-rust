mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::cohorts::{CreateCohortParams, GetCohortParams, UpdateCohortParams};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_cohort() {
    let mut server = Server::new_async().await;
    let expected_description = "string";
    let expected_name = "a";
    let expected_organization_prn = "1";
    let expected_product_prn = "string";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/cohorts"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/cohorts-create-201.json")
        .create_async()
        .await;

    let params = CreateCohortParams {
        description: Some(expected_description.to_string()),
        name: expected_name.to_string(),
        product_prn: expected_product_prn.to_string(),
    };

    match api.cohorts().create(params).await.unwrap() {
        Some(cohort) => {
            assert_eq!(
                cohort.cohort.description,
                Some(expected_description.to_string())
            );
            assert_eq!(cohort.cohort.name, expected_name.to_string());
            assert_eq!(
                cohort.cohort.organization_prn,
                expected_organization_prn.to_string()
            );
            assert_eq!(cohort.cohort.product_prn, expected_product_prn.to_string());
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_cohort() {
    let mut server = Server::new_async().await;
    let expected_description = "string";
    let expected_name = "a";
    let expected_organization_prn = "1";
    let expected_prn = "2";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/cohorts/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/cohorts-get-200.json")
        .create_async()
        .await;

    let params = GetCohortParams {
        prn: expected_prn.to_string(),
    };

    match api.cohorts().get(params).await.unwrap() {
        Some(cohort) => {
            assert_eq!(
                cohort.cohort.description,
                Some(expected_description.to_string())
            );
            assert_eq!(cohort.cohort.name, expected_name.to_string());
            assert_eq!(
                cohort.cohort.organization_prn,
                expected_organization_prn.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_cohort() {
    let mut server = Server::new_async().await;
    let expected_description = "string";
    let expected_name = "a";
    let expected_organization_prn = "1";
    let expected_prn = "1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("PATCH", &*format!("/cohorts/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/cohorts-update-200.json")
        .create_async()
        .await;

    let params = UpdateCohortParams {
        prn: expected_prn.to_string(),
        description: Some(expected_description.to_string()),
        name: Some(expected_name.to_string()),
    };

    match api.cohorts().update(params).await.unwrap() {
        Some(cohort) => {
            assert_eq!(
                cohort.cohort.description,
                Some(expected_description.to_string())
            );
            assert_eq!(cohort.cohort.name, expected_name.to_string());
            assert_eq!(
                cohort.cohort.organization_prn,
                expected_organization_prn.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
