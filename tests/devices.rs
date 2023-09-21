mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};

use peridio_sdk::api::devices::{
    AuthenticateDeviceParams, CreateDeviceParams, DeleteDeviceParams, GetDeviceParams,
    ListDeviceParams, UpdateDeviceParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_device() {
    let organization_name = "org-1";
    let product_name = "pro-1";

    let expected_description = "test";
    let expected_healthy = false;
    let expected_identifier = "a";
    let expected_last_communication = "date";
    let expected_tags = vec!["tag-1".to_string(), "tag-2".to_string()];
    let expected_target = "test-target";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "POST",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices"),
    )
    .with_status(201)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/devices-create-201.json")
    .create();

    let params = CreateDeviceParams {
        product_name: product_name.to_string(),
        organization_name: organization_name.to_string(),
        description: Some(expected_description.to_string()),
        healthy: Some(expected_healthy),
        identifier: expected_identifier.to_string(),
        last_communication: Some(expected_last_communication.to_string()),
        tags: Some(expected_tags),
        target: Some(expected_target.to_string()),
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

    m.assert();
}

#[tokio::test]
async fn delete_device() {
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "a";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "DELETE",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}"),
    )
    .with_status(204)
    .with_body("")
    .create();

    let params = DeleteDeviceParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        device_identifier: device_identifier.to_string(),
    };

    match api.devices().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn get_device() {
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "a";

    let expected_description = "test";
    let expected_healthy = false;
    let expected_identifier = "a";
    let expected_last_communication = "date";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "GET",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/devices-get-200.json")
    .create();

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

    m.assert();
}

#[tokio::test]
async fn list_device() {
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
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "GET",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/devices-list-200.json")
    .create();

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

    m.assert();
}

#[tokio::test]
async fn update_device() {
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "a";

    let expected_description = "test-updated";
    let expected_healthy = true;
    let expected_identifier = "a";
    let expected_last_communication = "new-date";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "PUT",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/devices-update-200.json")
    .create();

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

    m.assert();
}

#[tokio::test]
async fn authenticate_device() {
    let organization_name = "org-1";
    let product_name = "pro-1";
    let certificate = "dGVzdA==";

    let expected_description = "test";
    let expected_healthy = false;
    let expected_identifier = "a";
    let expected_last_communication = "date";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "POST",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices/auth"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/devices-authenticate-200.json")
    .create();

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

    m.assert();
}
