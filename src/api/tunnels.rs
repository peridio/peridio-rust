use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, list_params::ListParams, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tunnel {
    pub cidr_block_allowlist: Option<Vec<String>>,
    pub device_prn: String,
    pub device_proxy_ip_address: Option<String>,
    pub device_proxy_port: Option<u16>,
    pub device_public_key: Option<String>,
    pub device_tunnel_port: u16,
    pub expires_at: String,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub server_proxy_ip_address: Option<String>,
    pub server_proxy_port: Option<u16>,
    pub server_public_key: Option<String>,
    pub server_tunnel_ip_address: Option<String>,
    pub server_tunnel_port: Option<u16>,
    pub state: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTunnelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cidr_block_allowlist: Option<Vec<String>>,
    pub device_prn: String,
    pub device_tunnel_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ttl: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTunnelResponse {
    pub tunnel: Tunnel,
}

#[derive(Debug, Serialize)]
pub struct GetTunnelParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTunnelResponse {
    pub tunnel: Tunnel,
}

#[derive(Debug, Serialize)]
pub struct ListTunnelsParams {
    #[serde(flatten)]
    pub list: ListParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListTunnelsResponse {
    pub tunnels: Vec<Tunnel>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTunnelParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ttl: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTunnelResponse {
    pub tunnel: Tunnel,
}

pub struct TunnelsApi<'a>(pub &'a Api);

impl<'a> TunnelsApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateTunnelParams,
    ) -> Result<Option<CreateTunnelResponse>, Error> {
        self.0
            .execute(Method::POST, "/tunnels", Some(json_body!(&params)))
            .await
    }

    pub async fn get(
        &'a self,
        params: GetTunnelParams,
    ) -> Result<Option<GetTunnelResponse>, Error> {
        let tunnel_prn: String = params.prn;
        self.0
            .execute(Method::GET, format!("/tunnels/{tunnel_prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListTunnelsParams,
    ) -> Result<Option<ListTunnelsResponse>, Error> {
        self.0
            .execute_with_params(
                Method::GET,
                "/tunnels".to_string(),
                None,
                params.list.to_query_params(),
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateTunnelParams,
    ) -> Result<Option<UpdateTunnelResponse>, Error> {
        let tunnel_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/tunnels/{tunnel_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
