mod common;

use common::API_KEY;
use mockito::{mock, server_url as mock_server_url};

use peridio_sdk::api::tunnels::{CreateTunnelParams, UpdateTunnelParams};

use peridio_sdk::api::Api;
use peridio_sdk::api::ApiOptions;

#[tokio::test]
async fn create_tunnel() {
    let cidr_block_allowlist = ["10.0.0.1/32".to_string()].to_vec();
    let device_prn = "device_prn";
    let port = 22;
    let ttl = 3600;

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("POST", &*format!("/tunnels"))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/tunnels-create-201.json")
        .create();

    let params = CreateTunnelParams {
        cidr_block_allowlist: Some(cidr_block_allowlist.clone()),
        device_prn: device_prn.to_string(),
        device_tunnel_port: port,
        ttl: Some(ttl),
    };

    match api.tunnels().create(params).await.unwrap() {
        Some(tunnel) => {
            assert_eq!(tunnel.tunnel.expires_at, "2000-01-01T00:00:00Z");
            assert_eq!(tunnel.tunnel.device_tunnel_port, port);
            assert_eq!(tunnel.tunnel.cidr_block_allowlist, cidr_block_allowlist);
        }
        _ => panic!(),
    }

    m.assert();
}

#[tokio::test]
async fn update_tunnel() {
    let expected_state = "closed";
    let expected_prn = "1";

    let api = Api::new(ApiOptions {
        api_key: API_KEY.into(),
        endpoint: Some(mock_server_url()),
        ca_bundle_path: None,
    });

    let m = mock("PATCH", &*format!("/tunnels/{expected_prn}"))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/fixtures/tunnels-update-200.json")
        .create();

    let params = UpdateTunnelParams {
        prn: expected_prn.to_string(),
        state: Some(expected_state.to_string()),
    };

    match api.tunnels().update(params).await.unwrap() {
        Some(tunnel) => {
            assert_eq!(tunnel.tunnel.state, expected_state.to_string());
        }
        _ => panic!(),
    }

    m.assert();
}