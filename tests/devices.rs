mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::devices::{
    AuthenticateDeviceParams, CreateDeviceParams, DeleteDeviceParams, GetDeviceParams,
    GetUpdateDeviceParams, ListDeviceParams, UpdateDeviceParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_device() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";

    let expected_description = "test";
    let expected_healthy = false;
    let expected_identifier = "a";
    let expected_last_communication = "date";
    let expected_tags = vec!["tag-1".to_string(), "tag-2".to_string()];
    let expected_target = "test-target";
    let expected_cohort_prn = "a";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "POST",
            &*format!("/orgs/{organization_name}/products/{product_name}/devices"),
        )
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-create-201.json")
        .create_async()
        .await;

    let params = CreateDeviceParams {
        product_name: product_name.to_string(),
        organization_name: organization_name.to_string(),
        description: Some(expected_description.to_string()),
        healthy: Some(expected_healthy),
        identifier: expected_identifier.to_string(),
        last_communication: Some(expected_last_communication.to_string()),
        tags: Some(expected_tags),
        target: Some(expected_target.to_string()),
        cohort_prn: Some(expected_cohort_prn.to_string()),
    };

    match api.devices().create(params).await.unwrap() {
        Some(device) => {
            assert_eq!(
                device.data.description,
                Some(expected_description.to_string())
            );
            assert_eq!(device.data.healthy, Some(expected_healthy));
            assert_eq!(device.data.identifier, expected_identifier);
            assert_eq!(device.data.last_communication, expected_last_communication);
            assert_eq!(device.data.target, Some(expected_target.to_string()));
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_device() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "a";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "DELETE",
            &*format!(
                "/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}"
            ),
        )
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = DeleteDeviceParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        device_identifier: device_identifier.to_string(),
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
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "a";

    let expected_description = "test";
    let expected_healthy = false;
    let expected_identifier = "a";
    let expected_last_communication = "date";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "GET",
            &*format!(
                "/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}"
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-get-200.json")
        .create_async()
        .await;

    let params = GetDeviceParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        device_identifier: device_identifier.to_string(),
    };

    match api.devices().get(params).await.unwrap() {
        Some(device) => {
            assert_eq!(
                device.data.description,
                Some(expected_description.to_string())
            );
            assert_eq!(device.data.healthy, Some(expected_healthy));
            assert_eq!(device.data.identifier, expected_identifier);
            assert_eq!(device.data.last_communication, expected_last_communication);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn list_device() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";

    let expected_description_0 = "test";
    let expected_healthy_0 = false;
    let expected_identifier_0 = "a";

    let expected_description_1 = "test-2";
    let expected_healthy_1 = true;
    let expected_identifier_1 = "b";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "GET",
            &*format!("/orgs/{organization_name}/products/{product_name}/devices"),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-list-200.json")
        .create_async()
        .await;

    let params = ListDeviceParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
    };

    match api.devices().list(params).await.unwrap() {
        Some(devices) => {
            assert_eq!(devices.data.len(), 2);

            assert_eq!(
                devices.data[0].description,
                Some(expected_description_0.to_string())
            );
            assert_eq!(devices.data[0].healthy, Some(expected_healthy_0));
            assert_eq!(devices.data[0].identifier, expected_identifier_0);

            assert_eq!(
                devices.data[1].description,
                Some(expected_description_1.to_string())
            );
            assert_eq!(devices.data[1].healthy, Some(expected_healthy_1));
            assert_eq!(devices.data[1].identifier, expected_identifier_1);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_device() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "a";

    let expected_description = "test-updated";
    let expected_healthy = true;
    let expected_identifier = "a";
    let expected_last_communication = "new-date";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "PUT",
            &*format!(
                "/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}"
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-update-200.json")
        .create_async()
        .await;

    let params = UpdateDeviceParams {
        product_name: product_name.to_string(),
        organization_name: organization_name.to_string(),
        device_identifier: device_identifier.to_string(),
        description: Some(expected_description.to_string()),
        healthy: Some(expected_healthy),
        last_communication: Some(expected_last_communication.to_string()),
        tags: None,
        target: None,
    };

    match api.devices().update(params).await.unwrap() {
        Some(device) => {
            assert_eq!(
                device.data.description,
                Some(expected_description.to_string())
            );
            assert_eq!(device.data.healthy, Some(expected_healthy));
            assert_eq!(device.data.identifier, expected_identifier);
            assert_eq!(device.data.last_communication, expected_last_communication);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn authenticate_device() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";
    let certificate = "dGVzdA==";

    let expected_description = "test";
    let expected_healthy = false;
    let expected_identifier = "a";
    let expected_last_communication = "date";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "POST",
            &*format!("/orgs/{organization_name}/products/{product_name}/devices/auth"),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-authenticate-200.json")
        .create_async()
        .await;

    let params = AuthenticateDeviceParams {
        product_name: product_name.to_string(),
        organization_name: organization_name.to_string(),
        certificate: certificate.to_string(),
    };

    match api.devices().authenticate(params).await.unwrap() {
        Some(device) => {
            assert_eq!(
                device.data.description,
                Some(expected_description.to_string())
            );
            assert_eq!(device.data.healthy, Some(expected_healthy));
            assert_eq!(device.data.identifier, expected_identifier);
            assert_eq!(device.data.last_communication, expected_last_communication);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_update_device() {
    let mut server = Server::new_async().await;
    let device_prn = "prn-1";
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
        .mock("POST", &*format!("/devices/{device_prn}/update"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/devices-get-update-200.json")
        .create_async()
        .await;

    let params = GetUpdateDeviceParams {
        device_prn: device_prn.to_string(),
        release_prn: release_prn.to_string().into(),
        bundle_prn: bundle_prn.to_string().into(),
        release_version: release_version.to_string().into(),
        write: false,
    };

    match api.devices().get_update(params).await.unwrap() {
        Some(device_update) => {
            assert_eq!(device_update.status, expected_update.to_string());
            assert_eq!(device_update.manifest.unwrap()[0].size, Some(expected_size));
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
