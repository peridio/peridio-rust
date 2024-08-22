mod common;

use common::API_KEY;
use mockito::Server;
use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn get_users_me_api() {
    let mut server = Server::new_async().await;
    let expected_email = "a@b.com";
    let expected_username = "c";
    let path = "/users/me".to_string();

    let m = server
        .mock("GET", &*path)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/users-me-200.json")
        .create_async()
        .await;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(server.url()),
        ca_bundle_path: None,
    });

    match api.users().me().await.unwrap() {
        Some(users_me) => {
            assert_eq!(users_me.data.email, expected_email);
            assert_eq!(users_me.data.username, expected_username);
        }
        _ => panic!(),
    }

    m.assert_async().await;
}
