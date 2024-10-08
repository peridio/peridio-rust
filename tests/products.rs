mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::products::{
    CreateProductParams, DeleteProductParams, GetProductParams, ListProductParams, UpdateProduct,
    UpdateProductParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_product() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";

    let expected_name = "pro-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("POST", &*format!("/orgs/{organization_name}/products"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-create-201.json")
        .create_async()
        .await;

    let params = CreateProductParams {
        name: expected_name.to_string(),
        organization_name: organization_name.to_string(),
    };

    match api.products().create(params).await.unwrap() {
        Some(product) => {
            assert_eq!(product.data.name, expected_name);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn delete_product() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "DELETE",
            &*format!("/orgs/{organization_name}/products/{product_name}"),
        )
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = DeleteProductParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
    };

    match api.products().delete(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_product() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";

    let expected_name = "pro-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "GET",
            &*format!("/orgs/{organization_name}/products/{product_name}"),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-get-200.json")
        .create_async()
        .await;

    let params = GetProductParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
    };

    match api.products().get(params).await.unwrap() {
        Some(product) => {
            assert_eq!(product.data.name, expected_name);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn list_products() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";

    let expected_name_0 = "pro-0";

    let expected_name_1 = "pro-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock("GET", &*format!("/orgs/{organization_name}/products"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-list-200.json")
        .create_async()
        .await;

    let params = ListProductParams {
        organization_name: organization_name.to_string(),
    };

    match api.products().list(params).await.unwrap() {
        Some(products) => {
            assert_eq!(products.data.len(), 2);

            assert_eq!(products.data[0].name, expected_name_0);

            assert_eq!(products.data[1].name, expected_name_1);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_product() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";

    let expected_name = "pro-2";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "PUT",
            &*format!("/orgs/{organization_name}/products/{product_name}"),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/products-update-200.json")
        .create_async()
        .await;

    let params = UpdateProductParams {
        product_name: product_name.to_string(),
        organization_name: organization_name.to_string(),
        product: UpdateProduct {
            name: Some(expected_name.to_string()),
        },
    };

    match api.products().update(params).await.unwrap() {
        Some(product) => {
            assert_eq!(product.data.name, expected_name);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
