mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};
use peridio_sdk::api::webhooks::CreateWebhookParams;
use peridio_sdk::api::webhooks::GetWebhookParams;
use peridio_sdk::api::webhooks::UpdateWebhookParams;
use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_webhook() {
    let expected_url = "https://peridio.com";
    let expected_state = "disabled";
    let expected_description = "description";
    let expected_organization_prn = "organization_prn";
    let expected_enabled_events = vec!["device.release_changed".to_string()];

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("POST", &*format!("/webhooks"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/webhooks-create-201.json")
        .create();

    let params = CreateWebhookParams {
        description: Some(expected_description.to_string()),
        enabled_events: Some(expected_enabled_events.clone()),
        url: expected_url.to_string(),
        organization_prn: expected_organization_prn.to_string(),
    };

    match api.webhooks().create(params).await.unwrap() {
        Some(webhook) => {
            assert_eq!(
                webhook.webhook.description,
                Some(expected_description.to_string())
            );
            assert_eq!(webhook.webhook.url, Some(expected_url.to_string()));
            assert_eq!(webhook.webhook.state, Some(expected_state.to_string()));
            assert_eq!(webhook.webhook.enabled_events, expected_enabled_events);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn get_webhook() {
    let expected_description = "description";
    let expected_prn = "prn";
    let expected_state = "enabled";
    let expected_url = "https://peridio.com";
    let expected_enabled_events = vec!["device.release_changed".to_string()];

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("GET", &*format!("/webhooks/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/webhooks-get-200.json")
        .create();

    let params = GetWebhookParams {
        prn: expected_prn.to_string(),
    };

    match api.webhooks().get(params).await.unwrap() {
        Some(webhook) => {
            assert_eq!(
                webhook.webhook.description,
                Some(expected_description.to_string())
            );
            assert_eq!(webhook.webhook.url, Some(expected_url.to_string()));
            assert_eq!(webhook.webhook.state, Some(expected_state.to_string()));
            assert_eq!(webhook.webhook.enabled_events, expected_enabled_events);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn update_webhook() {
    let expected_description = "description";
    let expected_prn = "prn";
    let expected_url = "https://peridio.com";
    let expected_state = "enabled";
    let expected_enabled_events = vec!["device.release_changed".to_string()];

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("PATCH", &*format!("/webhooks/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/webhooks-update-200.json")
        .create();

    let params = UpdateWebhookParams {
        prn: expected_prn.to_string(),
        enabled_events: Some(expected_enabled_events.clone()),
        description: Some(expected_description.to_string()),
        url: Some(expected_url.to_string()),
        state: Some(expected_state.to_string()),
    };

    match api.webhooks().update(params).await.unwrap() {
        Some(webhook) => {
            assert_eq!(
                webhook.webhook.description,
                Some(expected_description.to_string())
            );
            assert_eq!(webhook.webhook.state, Some(expected_state.to_string()));
            assert_eq!(webhook.webhook.enabled_events, expected_enabled_events);
            assert_eq!(webhook.webhook.url, Some(expected_url.to_string()));
        }
        _ => panic!(),
    }

    m.assert();
}
