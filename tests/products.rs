mod common;

use common::API_KEY;
use mockito::Server;
use peridio_sdk::api::products::CreateProductParams;
use peridio_sdk::api::products::GetProductParams;
use peridio_sdk::api::products::UpdateProductParams;
use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_product() {
    let mut server = Server::new_async().await;
    let expected_name = "name";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/products"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-v2-create-201.json")
        .create_async()
        .await;

    let params = CreateProductParams {
        archived: None,
        name: expected_name.to_string(),
    };

    match api.products().create(params).await.unwrap() {
        Some(product) => {
            assert_eq!(product.product.name, expected_name.to_string());
            assert!(!product.product.archived);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_product() {
    let mut server = Server::new_async().await;
    let expected_name = "name";
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/products/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-v2-get-200.json")
        .create_async()
        .await;

    let params = GetProductParams {
        prn: expected_prn.to_string(),
    };

    match api.products().get(params).await.unwrap() {
        Some(product) => {
            assert_eq!(product.product.name, expected_name.to_string());
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_product() {
    let mut server = Server::new_async().await;
    let expected_archived = true;
    let expected_name = "name";
    let expected_prn = "prn";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("PATCH", &*format!("/products/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-v2-update-200.json")
        .create_async()
        .await;

    let params = UpdateProductParams {
        prn: expected_prn.to_string(),
        name: Some(expected_name.to_string()),
        archived: Some(true),
    };

    match api.products().update(params).await.unwrap() {
        Some(product) => {
            assert_eq!(product.product.name, expected_name.to_string());
            assert_eq!(product.product.archived, expected_archived);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
