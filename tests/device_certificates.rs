mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::device_certificates::{
    CreateDeviceCertificateParams, DeleteDeviceCertificateParams, GetDeviceCertificateParams,
    ListDeviceCertificateParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_device_certificate() {
    let mut server = Server::new_async().await;
    let device_prn = "dev-prn-1";
    let cert = "cert-1";

    let expected_not_after = "then";
    let expected_not_before = "now";
    let expected_serial = "serial";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/device_certificates"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/device-certificates-create-201.json")
        .create_async()
        .await;

    let params = CreateDeviceCertificateParams {
        device_prn: device_prn.to_string(),
        certificate: cert.to_string(),
    };

    if let Some(response) = api.device_certificates().create(params).await.unwrap() {
        let device_cert = response.device_certificate;
        assert_eq!(device_cert.not_after, expected_not_after);
        assert_eq!(device_cert.not_before, expected_not_before);
        assert_eq!(device_cert.serial, expected_serial);
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_device_certificate() {
    let mut server = Server::new_async().await;
    let prn = "prn-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("DELETE", &*format!("/device_certificates/{prn}"))
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = DeleteDeviceCertificateParams {
        prn: prn.to_string(),
    };

    match api.device_certificates().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_device_certificate() {
    let mut server = Server::new_async().await;
    let prn = "prn-1";

    let expected_not_after = "then";
    let expected_not_before = "now";
    let expected_serial = "serial";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/device_certificates/{prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/device-certificates-get-200.json")
        .create_async()
        .await;

    let params = GetDeviceCertificateParams {
        prn: prn.to_string(),
    };

    if let Some(response) = api.device_certificates().get(params).await.unwrap() {
        let device_cert = response.device_certificate;
        assert_eq!(device_cert.not_after, expected_not_after);
        assert_eq!(device_cert.not_before, expected_not_before);
        assert_eq!(device_cert.serial, expected_serial);
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn list_device_certificate() {
    let mut server = Server::new_async().await;

    let expected_not_after_0 = "then-0";
    let expected_not_before_0 = "now-0";
    let expected_prn_0 = "prn-1";
    let expected_serial_0 = "serial-0";

    let expected_not_after_1 = "then-1";
    let expected_not_before_1 = "now-1";
    let expected_prn_1 = "prn-2";
    let expected_serial_1 = "serial-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/device_certificates"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/device-certificates-list-200.json")
        .create_async()
        .await;

    let params = ListDeviceCertificateParams::default();

    if let Some(response) = api.device_certificates().list(params).await.unwrap() {
        let device_certificates = response.device_certificates;
        assert_eq!(device_certificates.len(), 2);

        assert_eq!(device_certificates[0].not_after, expected_not_after_0);
        assert_eq!(device_certificates[0].not_before, expected_not_before_0);
        assert_eq!(device_certificates[0].prn, expected_prn_0);
        assert_eq!(device_certificates[0].serial, expected_serial_0);

        assert_eq!(device_certificates[1].not_after, expected_not_after_1);
        assert_eq!(device_certificates[1].not_before, expected_not_before_1);
        assert_eq!(device_certificates[1].prn, expected_prn_1);
        assert_eq!(device_certificates[1].serial, expected_serial_1);
    } else {
        panic!();
    }

    m.assert_async().await;
}
