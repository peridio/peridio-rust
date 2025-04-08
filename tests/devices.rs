mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::devices::{
    CreateDeviceParams, DeleteDeviceParams, GetDeviceParams, GetUpdateDeviceParams,
    ListDeviceParams, UpdateDeviceParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_device() {
    let mut server = Server::new_async().await;

    let product_prn =
        "prn:1:be4d30b4-de6b-47cd-85ea-a75e23fd63ef:product:b3f1f699-3bc8-4c77-bda2-b974595d5e3f";

    let expected_description = "some description";
    let expected_quarantined = false;
    let expected_identifier = "sn1234";
    let expected_tags = vec!["tag-1".to_string()];
    let expected_target = "arm-linux-androideabi";
    let expected_cohort_prn =
        "prn:1:be4d30b4-de6b-47cd-85ea-a75e23fd63ef:cohort:b3f1f699-3bc8-4c77-bda2-b974595d5e3f";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/devices"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-create-201.json")
        .create_async()
        .await;

    let params = CreateDeviceParams {
        product_prn: product_prn.to_string(),
        description: Some(expected_description.to_string()),
        quarantined: Some(expected_quarantined),
        identifier: expected_identifier.to_string(),
        tags: Some(expected_tags),
        target: Some(expected_target.to_string()),
        cohort_prn: Some(expected_cohort_prn.to_string()),
    };

    if let Some(response) = api.devices().create(params).await.unwrap() {
        let device = response.device;
        assert_eq!(device.description, Some(expected_description.to_string()));
        assert_eq!(device.quarantined, expected_quarantined);
        assert_eq!(device.identifier, expected_identifier);
        assert_eq!(device.target, Some(expected_target.to_string()));
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_device() {
    let mut server = Server::new_async().await;
    let prn = "prn-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("DELETE", &*format!("/devices/{prn}"))
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = DeleteDeviceParams {
        prn: prn.to_string(),
    };

    match api.devices().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_device() {
    let mut server = Server::new_async().await;
    let prn =
        "prn:1:be4d30b4-de6b-47cd-85ea-a75e23fd63ef:device:b3f1f699-3bc8-4c77-bda2-b974595d5e3f";

    let expected_description = "test";
    let expected_quarantined = false;
    let expected_identifier = "sn1234";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/devices/{prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-get-200.json")
        .create_async()
        .await;

    let params = GetDeviceParams {
        prn: prn.to_string(),
    };

    if let Some(response) = api.devices().get(params).await.unwrap() {
        let device = response.device;
        assert_eq!(device.description, Some(expected_description.to_string()));
        assert_eq!(device.quarantined, expected_quarantined);
        assert_eq!(device.identifier, expected_identifier);
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn list_devices() {
    let mut server = Server::new_async().await;

    let expected_description_0 = "some description";
    let expected_quarantined_0 = false;
    let expected_identifier_0 = "sn1234";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/devices"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-list-200.json")
        .create_async()
        .await;

    let params = ListDeviceParams::default();

    if let Some(response) = api.devices().list(params).await.unwrap() {
        let devices = response.devices;
        assert_eq!(devices.len(), 1);
        assert_eq!(
            devices[0].description,
            Some(expected_description_0.to_string())
        );
        assert_eq!(devices[0].quarantined, expected_quarantined_0);
        assert_eq!(devices[0].identifier, expected_identifier_0);
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_device() {
    let mut server = Server::new_async().await;
    let product_prn =
        "prn:1:be4d30b4-de6b-47cd-85ea-a75e23fd63ef:product:b3f1f699-3bc8-4c77-bda2-b974595d5e3f";
    let cohort_prn =
        "prn:1:be4d30b4-de6b-47cd-85ea-a75e23fd63ef:cohort:b3f1f699-3bc8-4c77-bda2-b974595d5e3f";
    let prn =
        "prn:1:be4d30b4-de6b-47cd-85ea-a75e23fd63ef:device:b3f1f699-3bc8-4c77-bda2-b974595d5e3f";

    let expected_description = "some description";
    let expected_quarantined = true;
    let expected_identifier = "sn1234";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("PATCH", &*format!("/devices/{prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-update-200.json")
        .create_async()
        .await;

    let params = UpdateDeviceParams {
        product_prn: Some(product_prn.to_string()),
        cohort_prn: Some(cohort_prn.to_string()),
        prn: prn.to_string(),
        description: Some(expected_description.to_string()),
        quarantined: Some(expected_quarantined),
        tags: None,
        target: None,
    };

    if let Some(response) = api.devices().update(params).await.unwrap() {
        let device = response.device;
        assert_eq!(device.description, Some(expected_description.to_string()));
        assert_eq!(device.quarantined, expected_quarantined);
        assert_eq!(device.identifier, expected_identifier);
        assert_eq!(device.cohort_prn, Some(cohort_prn.to_string()));
        assert_eq!(device.prn, prn);
        assert_eq!(device.product_prn, product_prn.to_string());
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_update_device() {
    let mut server = Server::new_async().await;
    let prn = "prn-1";
    let release_prn = "rel-1";
    let bundle_prn = "bun-1";
    let release_version = "rev-1";

    let expected_update = "update";
    let expected_size = 10u32;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/devices/{prn}/update"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-get-update-200.json")
        .create_async()
        .await;

    let params = GetUpdateDeviceParams {
        prn: prn.to_string(),
        release_prn: release_prn.to_string().into(),
        bundle_prn: bundle_prn.to_string().into(),
        release_version: release_version.to_string().into(),
        write: false,
    };

    if let Some(response) = api.devices().get_update(params).await.unwrap() {
        assert_eq!(response.status, expected_update.to_string());
        assert_eq!(response.manifest.unwrap()[0].size, Some(expected_size));
    } else {
        panic!();
    }

    m.assert_async().await;
}
