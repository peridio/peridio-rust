mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::firmwares::{
    CreateFirmwareParams, DeleteFirmwareParams, GetFirmwareParams, ListFirmwareParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_firmware() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let firmware = "tests/files/firmware_test";
    let ttl = 10;

    let expected_architecture = "arm64";
    let expected_author = "org-1";
    let expected_inserted_at = "2019-08-24T14:15:22Z";
    let expected_platform = "rpi";
    let expected_product = "test";
    let expected_updated_at = "2019-08-24T14:15:22Z";
    let expected_uuid = "4dd9ff49-ec74-45fc-8558-7f98839019ec";
    let expected_vcs_identifier = "d670460b4b4aece5915caf5c68d12f560a9fe3e4";
    let expected_version = "1.0.0";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "POST",
            &*format!("/orgs/{organization_name}/products/{expected_product}/firmwares"),
        )
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/firmwares-create-201.json")
        .create_async()
        .await;

    let params = CreateFirmwareParams {
        product_name: expected_product.to_string(),
        firmware_path: firmware.to_string(),
        ttl: Some(ttl),
        organization_name: organization_name.to_string(),
    };

    match api.firmwares().create(params).await.unwrap() {
        Some(firmware) => {
            assert_eq!(firmware.data.architecture, expected_architecture);
            assert_eq!(firmware.data.author, Some(expected_author.to_string()));
            assert_eq!(firmware.data.inserted_at, expected_inserted_at);
            assert_eq!(firmware.data.platform, expected_platform);
            assert_eq!(firmware.data.product, expected_product);
            assert_eq!(firmware.data.updated_at, expected_updated_at);
            assert_eq!(firmware.data.uuid, expected_uuid);
            assert_eq!(firmware.data.version, expected_version);
            assert_eq!(
                firmware.data.vcs_identifier,
                Some(expected_vcs_identifier.to_string())
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_firmware() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "test";
    let firmware_uuid = "4dd9ff49-ec74-45fc-8558-7f98839019ec";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "DELETE",
            &*format!(
                "/orgs/{organization_name}/products/{product_name}/firmwares/{firmware_uuid}"
            ),
        )
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = DeleteFirmwareParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        firmware_uuid: firmware_uuid.to_string(),
    };

    match api.firmwares().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_firmware() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "test";
    let firmware_uuid = "4dd9ff49-ec74-45fc-8558-7f98839019ec";

    let expected_architecture = "arm64";
    let expected_author = "org-1";
    let expected_inserted_at = "2019-08-24T14:15:22Z";
    let expected_platform = "rpi";
    let expected_product = "test";
    let expected_updated_at = "2019-08-24T14:15:22Z";
    let expected_uuid = "4dd9ff49-ec74-45fc-8558-7f98839019ec";
    let expected_vcs_identifier = "d670460b4b4aece5915caf5c68d12f560a9fe3e4";
    let expected_version = "1.0.0";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "GET",
            &*format!(
                "/orgs/{organization_name}/products/{product_name}/firmwares/{firmware_uuid}"
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/firmwares-get-200.json")
        .create_async()
        .await;

    let params = GetFirmwareParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        firmware_uuid: firmware_uuid.to_string(),
    };

    match api.firmwares().get(params).await.unwrap() {
        Some(firmware) => {
            assert_eq!(firmware.data.architecture, expected_architecture);
            assert_eq!(firmware.data.author, Some(expected_author.to_string()));
            assert_eq!(firmware.data.inserted_at, expected_inserted_at);
            assert_eq!(firmware.data.platform, expected_platform);
            assert_eq!(firmware.data.product, expected_product);
            assert_eq!(firmware.data.updated_at, expected_updated_at);
            assert_eq!(firmware.data.uuid, expected_uuid);
            assert_eq!(firmware.data.version, expected_version);
            assert_eq!(
                firmware.data.vcs_identifier,
                Some(expected_vcs_identifier.to_string())
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn list_firmwares() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "test";

    let expected_inserted_at_0 = "2019-08-24T14:15:22Z";
    let expected_updated_at_0 = "2019-08-24T14:15:22Z";
    let expected_uuid_0 = "4dd9ff49-ec74-45fc-8558-7f98839019ec";
    let expected_vcs_identifier_0 = "d670460b4b4aece5915caf5c68d12f560a9fe3e4";
    let expected_version_0 = "1.0.0";
    let expected_product_0 = "test";

    let expected_inserted_at_1 = "2020-11-04T16:20:00Z";
    let expected_updated_at_1 = "2020-11-04T16:20:00Z";
    let expected_uuid_1 = "1905eef3-f946-4b48-8584-dfc3e344ba87";
    let expected_vcs_identifier_1 = "d670460b4b4aece5915caf5c68d12f560a9fe3e4";
    let expected_version_1 = "2.0.0";
    let expected_product_1 = "test-2";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "GET",
            &*format!("/orgs/{organization_name}/products/{product_name}/firmwares"),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/firmwares-list-200.json")
        .create_async()
        .await;

    let params = ListFirmwareParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
    };

    match api.firmwares().list(params).await.unwrap() {
        Some(firmwares) => {
            assert_eq!(firmwares.data.len(), 2);
            assert_eq!(firmwares.data[0].inserted_at, expected_inserted_at_0);
            assert_eq!(firmwares.data[0].updated_at, expected_updated_at_0);
            assert_eq!(firmwares.data[0].uuid, expected_uuid_0);
            assert_eq!(firmwares.data[0].version, expected_version_0);
            assert_eq!(firmwares.data[0].product, expected_product_0);
            assert_eq!(
                firmwares.data[0].vcs_identifier,
                Some(expected_vcs_identifier_0.to_string())
            );

            assert_eq!(firmwares.data[1].inserted_at, expected_inserted_at_1);
            assert_eq!(firmwares.data[1].updated_at, expected_updated_at_1);
            assert_eq!(firmwares.data[1].uuid, expected_uuid_1);
            assert_eq!(firmwares.data[1].version, expected_version_1);
            assert_eq!(firmwares.data[1].product, expected_product_1);
            assert_eq!(
                firmwares.data[1].vcs_identifier,
                Some(expected_vcs_identifier_1.to_string())
            );
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
