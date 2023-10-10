mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};
use peridio_sdk::api::products_v2::CreateProductV2Params;
use peridio_sdk::api::products_v2::GetProductV2Params;
use peridio_sdk::api::products_v2::UpdateProductV2Params;
use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_product() {
    let expected_name = "name";
    let expected_organization_prn = "organization_prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("POST", &*format!("/products"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-v2-create-201.json")
        .create();

    let params = CreateProductV2Params {
        name: expected_name.to_string(),
        organization_prn: expected_organization_prn.to_string(),
    };

    match api.products_v2().create(params).await.unwrap() {
        Some(product) => {
            assert_eq!(product.product.name, expected_name.to_string());
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn get_product() {
    let expected_name = "name";
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("GET", &*format!("/products/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-v2-get-200.json")
        .create();

    let params = GetProductV2Params {
        prn: expected_prn.to_string(),
    };

    match api.products_v2().get(params).await.unwrap() {
        Some(product) => {
            assert_eq!(product.product.name, expected_name.to_string());
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn update_product() {
    let expected_archived = true;
    let expected_name = "name";
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("PATCH", &*format!("/products/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-v2-update-200.json")
        .create();

    let params = UpdateProductV2Params {
        prn: expected_prn.to_string(),
        name: Some(expected_name.to_string()),
        archived: Some(true),
    };

    match api.products_v2().update(params).await.unwrap() {
        Some(product) => {
            assert_eq!(product.product.name, expected_name.to_string());
            assert_eq!(product.product.archived, expected_archived);
        }
        _ => panic!(),
    }

    m.assert();
}
