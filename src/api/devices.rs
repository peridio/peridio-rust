use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{json_body, list_params::ListParams, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub cohort_prn: Option<String>,
    pub description: Option<String>,
    pub identifier: String,
    pub inserted_at: String,
    pub last_connected_at: Option<String>,
    pub prn: String,
    pub product_prn: String,
    pub quarantined: bool,
    pub reported_bundle_prn: Option<String>,
    pub reported_release_prn: Option<String>,
    pub reported_release_version: Option<String>,
    pub tags: Option<Vec<String>>,
    pub target: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceUpdate {
    pub status: String,
    pub bundle_prn: Option<String>,
    pub source_prn: Option<String>,
    pub source_type: Option<String>,
    pub manifest: Option<Vec<UpdateManifest>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateManifest {
    pub artifact_prn: Option<String>,
    pub artifact_version_prn: Option<String>,
    pub custom_metadata: Option<HashMap<String, serde_json::Value>>,
    pub prn: Option<String>,
    pub hash: Option<String>,
    pub signatures: Option<Vec<UpdateSignature>>,
    pub size: Option<u32>,
    pub target: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSignature {
    pub signing_key_prn: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct FirmwareMetadata {
    pub architecture: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub fwup_version: Option<String>,
    pub misc: Option<String>,
    pub platform: String,
    pub product: String,
    pub uuid: String,
    pub vcs_identifiers: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ListDeviceParams {
    #[serde(flatten)]
    pub list: ListParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListDeviceResponse {
    pub devices: Vec<Device>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetDeviceParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetDeviceResponse {
    pub device: Device,
}

#[derive(Debug, Serialize)]
pub struct DeleteDeviceParams {
    pub prn: String,
}

#[derive(Debug, Serialize)]
pub struct CreateDeviceParams {
    pub product_prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quarantined: Option<bool>,
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cohort_prn: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDeviceResponse {
    pub device: Device,
}

#[derive(Debug, Serialize)]
pub struct UpdateDeviceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub product_prn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cohort_prn: Option<String>,
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quarantined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDeviceResponse {
    pub device: Device,
}

#[derive(Debug, Serialize)]
pub struct AuthenticateDeviceParams {
    pub product_name: String,
    pub organization_name: String,
    pub certificate: String,
}

#[derive(Debug, Serialize)]
pub struct GetUpdateDeviceParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub release_prn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bundle_prn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub release_version: Option<String>,
    #[serde(default)]
    pub write: bool,
}

type GetUpdateDeviceResponse = DeviceUpdate;

pub struct DevicesApi<'a>(pub &'a Api);

impl<'a> DevicesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateDeviceParams,
    ) -> Result<Option<CreateDeviceResponse>, Error> {
        self.0
            .execute(
                Method::POST,
                "/devices".to_string(),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(&'a self, params: DeleteDeviceParams) -> Result<Option<()>, Error> {
        let prn = params.prn;

        self.0
            .execute(Method::DELETE, format!("/devices/{prn}"), None)
            .await
    }

    pub async fn get(
        &'a self,
        params: GetDeviceParams,
    ) -> Result<Option<GetDeviceResponse>, Error> {
        let prn = params.prn;

        self.0
            .execute(Method::GET, format!("/devices/{prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListDeviceParams,
    ) -> Result<Option<ListDeviceResponse>, Error> {
        self.0
            .execute_with_params(
                Method::GET,
                "/devices".to_string(),
                None,
                params.list.to_query_params(),
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateDeviceParams,
    ) -> Result<Option<UpdateDeviceResponse>, Error> {
        let prn = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/devices/{prn}"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn get_update(
        &'a self,
        params: GetUpdateDeviceParams,
    ) -> Result<Option<GetUpdateDeviceResponse>, Error> {
        let prn = &params.prn;

        self.0
            .execute(
                Method::POST,
                format!("/devices/{prn}/update"),
                Some(json_body!(&params)),
            )
            .await
    }
}
