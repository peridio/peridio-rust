use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub description: Option<String>,
    pub firmware_metadata: Option<FirmwareMetadata>,
    pub healthy: Option<bool>,
    pub identifier: String,
    pub last_communication: String,
    pub prn: String,
    pub status: String,
    pub tags: Option<Vec<String>>,
    pub version: String,
    pub target: Option<String>,
    pub cohort_prn: Option<String>,
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

#[derive(Debug, Serialize)]
pub struct ListDeviceParams {
    pub organization_name: String,
    pub product_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListDeviceResponse {
    pub data: Vec<Device>,
}

#[derive(Debug, Serialize)]
pub struct GetDeviceParams {
    pub product_name: String,
    pub organization_name: String,
    pub device_identifier: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetDeviceResponse {
    pub data: Device,
}

#[derive(Debug, Serialize)]
pub struct DeleteDeviceParams {
    pub product_name: String,
    pub organization_name: String,
    pub device_identifier: String,
}

#[derive(Debug, Serialize)]
pub struct CreateDeviceParams {
    pub product_name: String,
    pub organization_name: String,
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub healthy: Option<bool>,
    pub identifier: String,
    pub last_communication: Option<String>,
    pub tags: Option<Vec<String>>,
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cohort_prn: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDeviceResponse {
    pub data: Device,
}

#[derive(Debug, Serialize)]
pub struct UpdateDeviceParams {
    pub product_name: String,
    pub organization_name: String,
    pub device_identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub healthy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub last_communication: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDeviceResponse {
    pub data: Device,
}

#[derive(Debug, Serialize)]
pub struct AuthenticateDeviceParams {
    pub product_name: String,
    pub organization_name: String,
    pub certificate: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticateDeviceResponse {
    pub data: Device,
}

#[derive(Debug, Serialize)]
pub struct GetUpdateDeviceParams {
    pub device_prn: String,
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
        let product_name = &params.product_name;
        let organization_name = &params.organization_name;

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/products/{product_name}/devices"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(&'a self, params: DeleteDeviceParams) -> Result<Option<()>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;
        let device_identifier = params.device_identifier;

        self.0
            .execute(
                Method::DELETE,
                format!(
                    "/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}"
                ),
                None,
            )
            .await
    }

    pub async fn get(
        &'a self,
        params: GetDeviceParams,
    ) -> Result<Option<GetDeviceResponse>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;
        let device_identifier = params.device_identifier;

        self.0
            .execute(
                Method::GET,
                format!(
                    "/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}"
                ),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListDeviceParams,
    ) -> Result<Option<ListDeviceResponse>, Error> {
        let organization_name = params.organization_name;
        let product_name = params.product_name;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/products/{product_name}/devices"),
                None,
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateDeviceParams,
    ) -> Result<Option<UpdateDeviceResponse>, Error> {
        let organization_name = &params.organization_name;
        let product_name = &params.product_name;
        let device_identifier = &params.device_identifier;

        self.0
            .execute(
                Method::PUT,
                format!(
                    "/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}"
                ),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn authenticate(
        &'a self,
        params: AuthenticateDeviceParams,
    ) -> Result<Option<AuthenticateDeviceResponse>, Error> {
        let organization_name = &params.organization_name;
        let product_name = &params.product_name;

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/products/{product_name}/devices/auth"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn get_update(
        &'a self,
        params: GetUpdateDeviceParams,
    ) -> Result<Option<GetUpdateDeviceResponse>, Error> {
        let device_prn = &params.device_prn;

        self.0
            .execute(
                Method::POST,
                format!("/devices/{device_prn}/update"),
                Some(json_body!(&params)),
            )
            .await
    }
}
