mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};

use peridio_sdk::api::ca_certificates::{
    CaCertificateJitp, CreateCaCertificateParams, CreateVerificationCodeParams,
    DeleteCaCertificateParams, GetCaCertificateParams, ListCaCertificateParams,
    UpdateCaCertificateParams,
};

use peridio_sdk::api::{Api, ApiOptions};

#[tokio::test]
async fn create_ca_certificate() {
    let organization_name = "org-1";
    let certificate = "cert-base-64";
    let verification_certificate = "verification_cert-base-64";
    let description = "test";

    let jitp_description = "jitp-test";
    let jitp_product_name = "pro-1";
    let jitp_tags = vec!["tag1".to_string()];
    let jitp_cohort_prn: &str = "test";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "POST",
        &*format!("/orgs/{organization_name}/ca_certificates"),
    )
    .with_status(201)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/ca-certificates-create-201.json")
    .create();

    let jitp = CaCertificateJitp {
        description: jitp_description.to_string(),
        tags: jitp_tags,
        product_name: jitp_product_name.to_string(),
        cohort_prn: Some(jitp_cohort_prn.to_string()),
    };

    let params = CreateCaCertificateParams {
        organization_name: organization_name.to_string(),
        certificate: certificate.to_string(),
        verification_certificate: verification_certificate.to_string(),
        description: Some(description.to_string()),
        jitp: Some(jitp),
    };

    match api.ca_certificates().create(params).await.unwrap() {
        Some(ca_certificate) => {
            assert_eq!(
                ca_certificate.data.description,
                Some(description.to_string())
            );

            assert_eq!(
                ca_certificate.data.jitp.unwrap().description,
                jitp_description.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn delete_ca_certificate() {
    let organization_name = "org-1";
    let ca_certificate_serial = "ABCD-1234";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "DELETE",
        &*format!("/orgs/{organization_name}/ca_certificates/{ca_certificate_serial}"),
    )
    .with_status(204)
    .with_body("")
    .create();

    let params = DeleteCaCertificateParams {
        organization_name: organization_name.to_string(),
        ca_certificate_serial: ca_certificate_serial.to_string(),
    };

    match api.ca_certificates().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn get_ca_certificate() {
    let organization_name = "org-1";
    let ca_certificate_serial = "serial";

    let expected_description = "test";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "GET",
        &*format!("/orgs/{organization_name}/ca_certificates/{ca_certificate_serial}"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/ca-certificates-get-200.json")
    .create();

    let params = GetCaCertificateParams {
        organization_name: organization_name.to_string(),
        ca_certificate_serial: ca_certificate_serial.to_string(),
    };

    match api.ca_certificates().get(params).await.unwrap() {
        Some(ca_certificate) => {
            assert_eq!(
                ca_certificate.data.description,
                Some(expected_description.to_string())
            );
            assert_eq!(
                ca_certificate.data.serial,
                ca_certificate_serial.to_string()
            );
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn list_ca_certificate() {
    let organization_name = "org-1";

    let expected_description_0 = "test-0";
    let expected_serial_0 = "serial-0";

    let expected_description_1 = "test-1";
    let expected_serial_1 = "serial-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "GET",
        &*format!("/orgs/{organization_name}/ca_certificates"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/ca-certificates-list-200.json")
    .create();

    let params = ListCaCertificateParams {
        organization_name: organization_name.to_string(),
    };

    match api.ca_certificates().list(params).await.unwrap() {
        Some(ca_certificates) => {
            assert_eq!(ca_certificates.data.len(), 2);

            assert_eq!(
                ca_certificates.data[0].description,
                Some(expected_description_0.to_string())
            );
            assert_eq!(ca_certificates.data[0].serial, expected_serial_0);

            assert_eq!(
                ca_certificates.data[1].description,
                Some(expected_description_1.to_string())
            );
            assert_eq!(ca_certificates.data[1].serial, expected_serial_1);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn create_ca_verification_code() {
    let organization_name = "org-1";

    let expected_verification_id = "test";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "POST",
        &*format!("/orgs/{organization_name}/ca_certificates/verification_codes"),
    )
    .with_status(201)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/ca-certificates-verification-code-create-201.json")
    .create();

    let params = CreateVerificationCodeParams {
        organization_name: organization_name.to_string(),
    };

    match api
        .ca_certificates()
        .create_verification_code(params)
        .await
        .unwrap()
    {
        Some(verification_code) => {
            assert_eq!(
                verification_code.data.verification_code,
                expected_verification_id
            );
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn update_ca_certificate() {
    let organization_name = "org-1";
    let ca_certificate_serial = "serial";
    let jitp_description = "jitp-test";
    let jitp_product_name = "pro-1";
    let jitp_tags = vec!["tag1".to_string()];
    let jitp_cohort_prn: &str = "test";

    let description = "test-updated";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock(
        "PUT",
        &*format!("/orgs/{organization_name}/ca_certificates/{ca_certificate_serial}"),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body_from_file("tests/fixtures/ca-certificates-update-200.json")
    .create();

    let jitp = CaCertificateJitp {
        description: jitp_description.to_string(),
        tags: jitp_tags,
        product_name: jitp_product_name.to_string(),
        cohort_prn: Some(jitp_cohort_prn.to_string()),
    };

    let params = UpdateCaCertificateParams {
        organization_name: organization_name.to_string(),
        ca_certificate_serial: ca_certificate_serial.to_string(),
        description: Some(description.to_string()),
        jitp: Some(Some(jitp)),
    };

    match api.ca_certificates().update(params).await.unwrap() {
        Some(ca_certificate) => {
            let test_jitp: CaCertificateJitp = ca_certificate.data.jitp.unwrap();
            assert_eq!(
                ca_certificate.data.description,
                Some(description.to_string())
            );
            assert_eq!(test_jitp.description, jitp_description.to_string());
            assert_eq!(test_jitp.cohort_prn, Some(jitp_cohort_prn.to_string()),);
        }
        _ => panic!(),
    }

    m.assert();
}
