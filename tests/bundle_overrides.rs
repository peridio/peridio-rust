mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::bundle_overrides::{
    AddDeviceParams, CreateBundleOverrideParams, DeleteBundleOverrideParams,
    GetBundleOverrideParams, ListBundleOverridesParams, ListDevicesParams, RemoveDeviceParams,
    UpdateBundleOverrideParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_bundle_override() {
    let mut server = Server::new_async().await;
    let expected_organization_prn = "prn:1:099195e1-6810-46c1-9195-8c34f57744e9";
    let expected_name = "another override";
    let expected_description = "some description";
    let expected_bundle_prn =
        "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:bundle:7217e151-2cbe-4a6a-b763-52f7c8302707";
    let expected_starts_at = "2025-04-03T18:40:00.000000Z";
    let expected_ends_at = "2025-04-04T23:50:00.000000Z";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("POST", "/bundle_overrides")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-overrides-create-201.json")
        .create_async()
        .await;

    let params = CreateBundleOverrideParams {
        name: expected_name.to_string(),
        bundle_prn: expected_bundle_prn.to_string(),
        starts_at: expected_starts_at.to_string(),
        description: Some(expected_description.to_string()),
        ends_at: Some(expected_ends_at.to_string()),
    };

    match api.bundle_overrides().create(params).await.unwrap() {
        Some(bundle_override) => {
            assert_eq!(
                bundle_override.bundle_override.organization_prn,
                expected_organization_prn.to_string()
            );
            assert_eq!(
                bundle_override.bundle_override.name,
                expected_name.to_string()
            );
            assert_eq!(
                bundle_override.bundle_override.description,
                Some(expected_description.to_string())
            );
            assert_eq!(
                bundle_override.bundle_override.bundle_prn,
                expected_bundle_prn.to_string()
            );
            assert_eq!(
                bundle_override.bundle_override.starts_at,
                expected_starts_at.to_string()
            );
            assert_eq!(
                bundle_override.bundle_override.ends_at,
                Some(expected_ends_at.to_string())
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_bundle_override() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:bundle_override:61a5518b-7afb-4707-a611-b1a5c75904dc";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("DELETE", &*format!("/bundle_overrides/{expected_prn}"))
        .with_status(204)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let params = DeleteBundleOverrideParams {
        prn: expected_prn.to_string(),
    };

    match api.bundle_overrides().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_bundle_override() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:bundle_override:61a5518b-7afb-4707-a611-b1a5c75904dc";
    let expected_organization_prn = "prn:1:099195e1-6810-46c1-9195-8c34f57744e9";
    let expected_name = "another override";
    let expected_description = "some description";
    let expected_bundle_prn =
        "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:bundle:7217e151-2cbe-4a6a-b763-52f7c8302707";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("GET", &*format!("/bundle_overrides/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-overrides-get-200.json")
        .create_async()
        .await;

    let params = GetBundleOverrideParams {
        prn: expected_prn.to_string(),
    };

    match api.bundle_overrides().get(params).await.unwrap() {
        Some(bundle_override) => {
            assert_eq!(
                bundle_override.bundle_override.organization_prn,
                expected_organization_prn.to_string()
            );
            assert_eq!(
                bundle_override.bundle_override.name,
                expected_name.to_string()
            );
            assert_eq!(
                bundle_override.bundle_override.description,
                Some(expected_description.to_string())
            );
            assert_eq!(
                bundle_override.bundle_override.bundle_prn,
                expected_bundle_prn.to_string()
            );
            assert_eq!(
                bundle_override.bundle_override.prn,
                expected_prn.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn list_bundle_overrides() {
    let mut server = Server::new_async().await;

    let expected_name_0 = "another override";
    let expected_description_0 = "some description";
    let expected_name_1 = "test override without description";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("GET", "/bundle_overrides")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-overrides-list-200.json")
        .create_async()
        .await;

    let params = ListBundleOverridesParams::default();

    match api.bundle_overrides().list(params).await.unwrap() {
        Some(response) => {
            let bundle_overrides = response.bundle_overrides;
            assert_eq!(bundle_overrides.len(), 2);
            assert_eq!(bundle_overrides[0].name, expected_name_0.to_string());
            assert_eq!(
                bundle_overrides[0].description,
                Some(expected_description_0.to_string())
            );
            assert_eq!(bundle_overrides[1].name, expected_name_1.to_string());
            assert_eq!(bundle_overrides[1].description, None);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_bundle_override() {
    let mut server = Server::new_async().await;
    let expected_prn = "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:bundle_override:61a5518b-7afb-4707-a611-b1a5c75904dc";
    let expected_name = "updated override name";
    let expected_description = "updated description";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock("PATCH", &*format!("/bundle_overrides/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-overrides-update-200.json")
        .create_async()
        .await;

    let params = UpdateBundleOverrideParams {
        prn: expected_prn.to_string(),
        name: Some(expected_name.to_string()),
        description: Some(expected_description.to_string()),
        ends_at: Some("2025-04-05T23:59:00.000000Z".to_string()),
        starts_at: None,
        bundle_prn: None,
    };

    match api.bundle_overrides().update(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.bundle_override.name, expected_name.to_string());
            assert_eq!(
                response.bundle_override.description,
                Some(expected_description.to_string())
            );
            assert_eq!(response.bundle_override.prn, expected_prn.to_string());
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn list_bundle_override_devices() {
    let mut server = Server::new_async().await;
    let expected_bundle_override_prn = "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:bundle_override:61a5518b-7afb-4707-a611-b1a5c75904dc";
    let expected_device_prn_0 =
        "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:device:7851db83-381f-4cba-889c-41a70a5030bb";
    let expected_device_prn_1 =
        "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:device:728b9968-e961-413b-b7e4-300a07b92402";
    let expected_inserted_at = "2025-06-03T19:58:56.288438Z";
    let expected_updated_at = "2025-06-03T19:58:56.288438Z";
    let expected_next_page =
        "ZGVzYzppbnNlcnRlZF9hdDonMjAyNS0wNi0wM1QxOTo1ODo1Ni4yODg0MzhaJyxkZXNjOmlkOjE2OA==";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock(
            "GET",
            &*format!("/bundle_overrides/{expected_bundle_override_prn}/devices"),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-overrides-list-devices-200.json")
        .create_async()
        .await;

    let params = ListDevicesParams {
        prn: expected_bundle_override_prn.to_string(),
        ..Default::default()
    };

    match api.bundle_overrides().list_devices(params).await.unwrap() {
        Some(response) => {
            let devices = response.devices;
            assert_eq!(devices.len(), 10);
            assert_eq!(devices[0].device_prn, expected_device_prn_0.to_string());
            assert_eq!(devices[0].inserted_at, expected_inserted_at.to_string());
            assert_eq!(devices[0].updated_at, expected_updated_at.to_string());
            assert_eq!(devices[1].device_prn, expected_device_prn_1.to_string());
            assert_eq!(devices[1].inserted_at, expected_inserted_at.to_string());
            assert_eq!(devices[1].updated_at, expected_updated_at.to_string());
            assert_eq!(response.next_page, Some(expected_next_page.to_string()));
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn add_bundle_override_device() {
    let mut server = Server::new_async().await;
    let expected_bundle_override_prn = "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:bundle_override:61a5518b-7afb-4707-a611-b1a5c75904dc";
    let expected_device_prn =
        "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:device:7851db83-381f-4cba-889c-41a70a5030bb";
    let expected_inserted_at = "2025-06-03T19:58:56.288438Z";
    let expected_updated_at = "2025-06-03T19:58:56.288438Z";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock(
            "POST",
            &*format!("/bundle_overrides/{expected_bundle_override_prn}/devices"),
        )
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/bundle-overrides-add-device-201.json")
        .create_async()
        .await;

    let params = AddDeviceParams {
        prn: expected_bundle_override_prn.to_string(),
        device_prn: expected_device_prn.to_string(),
    };

    match api.bundle_overrides().add_device(params).await.unwrap() {
        Some(response) => {
            assert_eq!(response.device.device_prn, expected_device_prn.to_string());
            assert_eq!(
                response.device.inserted_at,
                expected_inserted_at.to_string()
            );
            assert_eq!(response.device.updated_at, expected_updated_at.to_string());
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn remove_bundle_override_device() {
    let mut server = Server::new_async().await;
    let expected_bundle_override_prn = "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:bundle_override:61a5518b-7afb-4707-a611-b1a5c75904dc";
    let expected_device_prn =
        "prn:1:099195e1-6810-46c1-9195-8c34f57744e9:device:7851db83-381f-4cba-889c-41a70a5030bb";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
        api_version: 1,
    });

    let m = server
        .mock(
            "DELETE",
            &*format!(
                "/bundle_overrides/{expected_bundle_override_prn}/devices/{expected_device_prn}"
            ),
        )
        .with_status(204)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let params = RemoveDeviceParams {
        prn: expected_bundle_override_prn.to_string(),
        device_prn: expected_device_prn.to_string(),
    };

    match api.bundle_overrides().remove_device(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}
