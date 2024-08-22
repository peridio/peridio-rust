mod common;

use common::API_KEY;
use mockito::Server;

use peridio_sdk::api::product_users::{
    AddProductUserParams, GetProductUserParams, ListProductUserParams, RemoveProductUserParams,
    UpdateProductUserParams,
};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn add_product_user() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";

    let expected_email = "test@test.com";
    let expected_role = "admin";
    let expected_username = "usr-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "POST",
            &*format!("/orgs/{organization_name}/products/{product_name}/users"),
        )
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/product-users-create-201.json")
        .create_async()
        .await;

    let params = AddProductUserParams {
        organization_name: organization_name.to_string(),
        role: expected_role.to_string(),
        username: expected_username.to_string(),
        product_name: product_name.to_string(),
    };

    match api.product_users().add(params).await.unwrap() {
        Some(product_user) => {
            assert_eq!(product_user.data.email, expected_email);
            assert_eq!(product_user.data.role, expected_role);
            assert_eq!(product_user.data.username, expected_username);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn remove_product_user() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";
    let user_username = "usr-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "DELETE",
            &*format!("/orgs/{organization_name}/products/{product_name}/users/{user_username}"),
        )
        .with_status(204)
        .with_body("")
        .create_async()
        .await;

    let params = RemoveProductUserParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        user_username: user_username.to_string(),
    };

    match api.product_users().remove(params).await.unwrap() {
        None => (),
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn get_product_user() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";
    let user_username = "usr-1";

    let expected_email = "test@test.com";
    let expected_role = "admin";
    let expected_username = "usr-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "GET",
            &*format!("/orgs/{organization_name}/products/{product_name}/users/{user_username}"),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/product-users-get-200.json")
        .create_async()
        .await;

    let params = GetProductUserParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
        user_username: user_username.to_string(),
    };

    match api.product_users().get(params).await.unwrap() {
        Some(organization_user) => {
            assert_eq!(organization_user.data.email, expected_email);
            assert_eq!(organization_user.data.role, expected_role);
            assert_eq!(organization_user.data.username, expected_username);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn list_product_user() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";

    let expected_email_0 = "test-0@test.com";
    let expected_role_0 = "admin-0";
    let expected_username_0 = "usr-0";

    let expected_email_1 = "test-1@test.com";
    let expected_role_1 = "admin-1";
    let expected_username_1 = "usr-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "GET",
            &*format!("/orgs/{organization_name}/products/{product_name}/users"),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/product-users-list-200.json")
        .create_async()
        .await;

    let params = ListProductUserParams {
        organization_name: organization_name.to_string(),
        product_name: product_name.to_string(),
    };

    match api.product_users().list(params).await.unwrap() {
        Some(product_users) => {
            assert_eq!(product_users.data.len(), 2);

            assert_eq!(product_users.data[0].email, expected_email_0);
            assert_eq!(product_users.data[0].role, expected_role_0);
            assert_eq!(product_users.data[0].username, expected_username_0);

            assert_eq!(product_users.data[1].email, expected_email_1);
            assert_eq!(product_users.data[1].role, expected_role_1);
            assert_eq!(product_users.data[1].username, expected_username_1);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}

#[tokio::test]
async fn update_product_user() {
    let mut server = Server::new_async().await;
    let organization_name = "org-1";
    let product_name = "pro-1";
    let user_username = "usr-1";

    let expected_email = "test@test.com";
    let expected_role = "user";
    let expected_username = "usr-1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    let m = server
        .mock(
            "PUT",
            &*format!("/orgs/{organization_name}/products/{product_name}/users/{user_username}"),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/product-users-update-200.json")
        .create_async()
        .await;

    let params = UpdateProductUserParams {
        organization_name: organization_name.to_string(),
        user_username: user_username.to_string(),
        role: expected_role.to_string(),
        product_name: product_name.to_string(),
    };

    match api.product_users().update(params).await.unwrap() {
        Some(product_user) => {
            assert_eq!(product_user.data.email, expected_email);
            assert_eq!(product_user.data.role, expected_role);
            assert_eq!(product_user.data.username, expected_username);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
