mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};

use peridio_sdk::api::device_certificates::{
    CreateDeviceCertificateParams, DeleteDeviceCertificateParams, GetDeviceCertificateParams,
    ListDeviceCertificateParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_device_certificate() {
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "dev-id-1";
    let cert = "cert-1";

    let expected_not_after = "then";
    let expected_not_before = "now";
    let expected_serial = "serial";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "POST",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}/certificates"),
    )
    .with_status(201)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/device-certificates-create-201.json")
    .create();

    let params = CreateDeviceCertificateParams {
        product_name: product_name.to_string(),
        organization_name: organization_name.to_string(),
        device_identifier: device_identifier.to_string(),
        cert: cert.to_string(),
    };

    match api.device_certificates().create(params).await.unwrap() {
        Some(device_certificate) => {
            assert_eq!(device_certificate.data.not_after, expected_not_after);
            assert_eq!(device_certificate.data.not_before, expected_not_before);
            assert_eq!(device_certificate.data.serial, expected_serial);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn delete_device_certificate() {
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "a";
    let certificate_serial = "serial";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "DELETE",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}/certificates/{certificate_serial}"),
    )
    .with_status(204)
    .with_body("")
    .create();

    let params = DeleteDeviceCertificateParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        device_identifier: device_identifier.to_string(),
        certificate_serial: certificate_serial.to_string(),
    };

    match api.device_certificates().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn get_device_certificate() {
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "a";
    let certificate_serial = "serial";

    let expected_not_after = "then";
    let expected_not_before = "now";
    let expected_serial = "serial";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "GET",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}/certificates/{certificate_serial}"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/device-certificates-get-200.json")
    .create();

    let params = GetDeviceCertificateParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        device_identifier: device_identifier.to_string(),
        certificate_serial: certificate_serial.to_string(),
    };

    match api.device_certificates().get(params).await.unwrap() {
        Some(device_certificate) => {
            assert_eq!(device_certificate.data.not_after, expected_not_after);
            assert_eq!(device_certificate.data.not_before, expected_not_before);
            assert_eq!(device_certificate.data.serial, expected_serial);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn list_device_certificate() {
    let organization_name = "org-1";
    let product_name = "pro-1";
    let device_identifier = "a";

    let expected_not_after_0 = "then-0";
    let expected_not_before_0 = "now-0";
    let expected_serial_0 = "serial-0";

    let expected_not_after_1 = "then-1";
    let expected_not_before_1 = "now-1";
    let expected_serial_1 = "serial-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "GET",
        &*format!("/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}/certificates"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/device-certificates-list-200.json")
    .create();

    let params = ListDeviceCertificateParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        device_identifier: device_identifier.to_string(),
    };

    match api.device_certificates().list(params).await.unwrap() {
        Some(device_certificate) => {
            assert_eq!(device_certificate.data.len(), 2);

            assert_eq!(device_certificate.data[0].not_after, expected_not_after_0);
            assert_eq!(device_certificate.data[0].not_before, expected_not_before_0);
            assert_eq!(device_certificate.data[0].serial, expected_serial_0);

            assert_eq!(device_certificate.data[1].not_after, expected_not_after_1);
            assert_eq!(device_certificate.data[1].not_before, expected_not_before_1);
            assert_eq!(device_certificate.data[1].serial, expected_serial_1);
        }
        _ => panic!(),
    }

    m.assert();
}
