mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::releases::{
    CreateReleaseParams, DeleteReleaseParams, GetReleaseParams, UpdateReleaseParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_release() {
    let mut server = Server::new_async().await;
    let expected_bundle_prn = "bundle_prn";
    let expected_cohort_prn = "cohort_prn";
    let expected_description = "description";
    let expected_next_release_prn = "next_release_prn";
    let expected_name = "name";
    let expected_organization_prn = "organization_prn";
    let expected_previous_release_prn = "previous_release_prn";
    let expected_phase_value = 1.0;
    let expected_required = true;
    let expected_schedule_date = "2000-01-01T00:00:00Z";
    let expected_version = "1.0.0";
    let expected_version_requirement = "= 1.0.0";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*"/releases".to_string())
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/releases-create-201.json")
        .create_async()
        .await;

    let params = CreateReleaseParams {
        bundle_prn: expected_bundle_prn.to_string(),
        cohort_prn: expected_cohort_prn.to_string(),
        description: Some(expected_description.to_string()),
        disabled: None,
        name: expected_name.to_string(),
        next_release_prn: Some(expected_next_release_prn.to_string()),
        organization_prn: expected_organization_prn.to_string(),
        phase_mode: None,
        phase_tags: None,
        phase_value: expected_phase_value.into(),
        previous_release_prn: Some(expected_previous_release_prn.to_string()),
        required: expected_required,
        schedule_date: expected_schedule_date.to_string(),
        version: Some(expected_version.to_string()),
        version_requirement: Some(expected_version_requirement.to_string()),
    };

    match api.releases().create(params).await.unwrap() {
        Some(release) => {
            assert_eq!(release.release.bundle_prn, expected_bundle_prn.to_string());
            assert_eq!(release.release.cohort_prn, expected_cohort_prn.to_string());
            assert_eq!(
                release.release.description,
                Some(expected_description.to_string())
            );
            assert_eq!(release.release.name, expected_name.to_string());
            assert_eq!(
                release.release.next_release_prn,
                Some(expected_next_release_prn.to_string())
            );
            assert_eq!(
                release.release.organization_prn,
                expected_organization_prn.to_string()
            );
            assert_eq!(release.release.phase_value, Some(expected_phase_value));
            assert_eq!(release.release.required, expected_required);
            assert_eq!(
                release.release.schedule_date,
                expected_schedule_date.to_string()
            );
            assert_eq!(release.release.version, Some(expected_version.to_string()));
            assert_eq!(
                release.release.version_requirement,
                Some(expected_version_requirement.to_string())
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_release() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("DELETE", &*format!("/releases/{expected_prn}"))
        .with_status(204)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let params = DeleteReleaseParams {
        prn: expected_prn.to_string(),
    };

    match api.releases().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_release() {
    let mut server = Server::new_async().await;
    let expected_bundle_prn = "bundle_prn";
    let expected_cohort_prn = "cohort_prn";
    let expected_description = "description";
    let expected_next_release_prn = "next_release_prn";
    let expected_name = "name";
    let expected_organization_prn = "organization_prn";
    let expected_prn = "prn";
    let expected_phase_value = 1.0;
    let expected_required = true;
    let expected_schedule_date = "2000-01-01T00:00:00Z";
    let expected_version = "1.0.0";
    let expected_version_requirement = "= 1.0.0";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/releases/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/releases-get-200.json")
        .create_async()
        .await;

    let params = GetReleaseParams {
        prn: expected_prn.to_string(),
    };

    match api.releases().get(params).await.unwrap() {
        Some(release) => {
            assert_eq!(release.release.bundle_prn, expected_bundle_prn.to_string());
            assert_eq!(release.release.cohort_prn, expected_cohort_prn.to_string());
            assert_eq!(
                release.release.description,
                Some(expected_description.to_string())
            );
            assert_eq!(release.release.name, expected_name.to_string());
            assert_eq!(
                release.release.next_release_prn,
                Some(expected_next_release_prn.to_string())
            );
            assert_eq!(
                release.release.organization_prn,
                expected_organization_prn.to_string()
            );
            assert_eq!(release.release.phase_value, Some(expected_phase_value));
            assert_eq!(release.release.required, expected_required);
            assert_eq!(
                release.release.schedule_date,
                expected_schedule_date.to_string()
            );
            assert_eq!(release.release.version, Some(expected_version.to_string()));
            assert_eq!(
                release.release.version_requirement,
                Some(expected_version_requirement.to_string())
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_release() {
    let mut server = Server::new_async().await;
    let expected_bundle_prn = "bundle_prn";
    let expected_cohort_prn = "cohort_prn";
    let expected_description = "updated_description";
    let expected_next_release_prn = "next_release_prn";
    let expected_name = "updated_name";
    let expected_organization_prn = "organization_prn";
    let expected_prn = "prn";
    let expected_phase_value = 0.5;
    let expected_required = false;
    let expected_schedule_date = "2001-01-01T00:00:00Z";
    let expected_version = "1.0.1";
    let expected_version_requirement = "= 1.0.1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("PATCH", &*format!("/releases/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/releases-update-200.json")
        .create_async()
        .await;

    let params = UpdateReleaseParams {
        prn: expected_prn.to_string(),
        description: Some(expected_description.to_string()),
        disabled: None,
        name: Some(expected_name.to_string()),
        next_release_prn: Some(expected_next_release_prn.to_string()),
        phase_mode: None,
        phase_tags: None,
        phase_value: Some(expected_phase_value),
        required: Some(expected_required),
        schedule_date: Some(expected_schedule_date.to_string()),
        version: Some(expected_version.to_string()),
        version_requirement: Some(expected_version_requirement.to_string()),
    };

    match api.releases().update(params).await.unwrap() {
        Some(release) => {
            assert_eq!(release.release.bundle_prn, expected_bundle_prn.to_string());
            assert_eq!(release.release.cohort_prn, expected_cohort_prn.to_string());
            assert_eq!(
                release.release.description,
                Some(expected_description.to_string())
            );
            assert_eq!(release.release.name, expected_name.to_string());
            assert_eq!(
                release.release.next_release_prn,
                Some(expected_next_release_prn.to_string())
            );
            assert_eq!(
                release.release.organization_prn,
                expected_organization_prn.to_string()
            );
            assert_eq!(release.release.phase_value, Some(expected_phase_value));
            assert_eq!(release.release.required, expected_required);
            assert_eq!(
                release.release.schedule_date,
                expected_schedule_date.to_string()
            );
            assert_eq!(release.release.version, Some(expected_version.to_string()));
            assert_eq!(
                release.release.version_requirement,
                Some(expected_version_requirement.to_string())
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
