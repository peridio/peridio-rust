mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::ca_certificates::{
    CreateCaCertificateParams, CreateVerificationCodeParams, DeleteCaCertificateParams,
    GetCaCertificateParams, ListCaCertificateParams, UpdateCaCertificateParams,
};

use peridio_sdk::api::{Api, ApiOptions};

#[tokio::test]
async fn create_ca_certificate() {
    let mut server = Server::new_async().await;
    let certificate = "cert-base-64";
    let verification_certificate = "verification_cert-base-64";
    let description = "test";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/ca_certificates"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/ca-certificates-create-201.json")
        .create_async()
        .await;

    let params = CreateCaCertificateParams {
        certificate: certificate.to_string(),
        verification_certificate: verification_certificate.to_string(),
        description: Some(description.to_string()),
    };

    if let Some(response) = api.ca_certificates().create(params).await.unwrap() {
        assert_eq!(
            response.ca_certificate.description,
            Some(description.to_string())
        );
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_ca_certificate() {
    let mut server = Server::new_async().await;
    let prn = "prn-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("DELETE", &*format!("/ca_certificates/{prn}"))
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = DeleteCaCertificateParams {
        prn: prn.to_string(),
    };

    match api.ca_certificates().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_ca_certificate() {
    let mut server = Server::new_async().await;
    let prn = "prn-1";

    let expected_description = "test";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/ca_certificates/{prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/ca-certificates-get-200.json")
        .create_async()
        .await;

    let params = GetCaCertificateParams {
        prn: prn.to_string(),
    };

    if let Some(response) = api.ca_certificates().get(params).await.unwrap() {
        assert_eq!(
            response.ca_certificate.description,
            Some(expected_description.to_string())
        );
        assert_eq!(response.ca_certificate.prn, prn.to_string());
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn list_ca_certificate() {
    let mut server = Server::new_async().await;

    let expected_description_0 = "test-0";
    let expected_serial_0 = "serial-0";

    let expected_description_1 = "test-1";
    let expected_serial_1 = "serial-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/ca_certificates"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/ca-certificates-list-200.json")
        .create_async()
        .await;

    let params = ListCaCertificateParams::default();

    if let Some(response) = api.ca_certificates().list(params).await.unwrap() {
        let ca_certs = response.ca_certificates;
        assert_eq!(ca_certs.len(), 2);

        assert_eq!(
            ca_certs[0].description,
            Some(expected_description_0.to_string())
        );
        assert_eq!(ca_certs[0].serial, expected_serial_0);

        assert_eq!(
            ca_certs[1].description,
            Some(expected_description_1.to_string())
        );
        assert_eq!(ca_certs[1].serial, expected_serial_1);
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn create_ca_verification_code() {
    let mut server = Server::new_async().await;

    let expected_verification_id = "test";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/ca_certificates/verification_codes"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/ca-certificates-verification-code-create-201.json")
        .create_async()
        .await;

    let params = CreateVerificationCodeParams {};

    if let Some(response) = api
        .ca_certificates()
        .create_verification_code(params)
        .await
        .unwrap()
    {
        assert_eq!(response.verification_code, expected_verification_id);
    } else {
        panic!();
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_ca_certificate() {
    let mut server = Server::new_async().await;
    let prn = "prn-1";

    let description = "test-updated";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("PATCH", &*format!("/ca_certificates/{prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/ca-certificates-update-200.json")
        .create_async()
        .await;

    let params = UpdateCaCertificateParams {
        prn: prn.to_string(),
        description: Some(description.to_string()),
    };

    if let Some(response) = api.ca_certificates().update(params).await.unwrap() {
        assert_eq!(
            response.ca_certificate.description,
            Some(description.to_string())
        );
    } else {
        panic!();
    }

    m.assert_async().await;
}
