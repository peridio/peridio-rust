use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, list_params::ListParams, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct BundleOverride {
    pub name: String,
    pub description: Option<String>,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub updated_at: String,
    pub ends_at: Option<String>,
    pub starts_at: String,
    pub bundle_prn: String,
}

#[derive(Debug, Serialize)]
pub struct CreateBundleOverrideParams {
    pub name: String,
    pub bundle_prn: String,
    pub starts_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBundleOverrideResponse {
    pub bundle_override: BundleOverride,
}

#[derive(Debug, Serialize)]
pub struct GetBundleOverrideParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetBundleOverrideResponse {
    pub bundle_override: BundleOverride,
}

#[derive(Debug, Serialize)]
pub struct DeleteBundleOverrideParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteBundleOverrideResponse {}

#[derive(Debug, Serialize, Default)]
pub struct ListBundleOverridesParams {
    #[serde(flatten)]
    pub list: ListParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListBundleOverridesResponse {
    pub bundle_overrides: Vec<BundleOverride>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateBundleOverrideParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_prn: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBundleOverrideResponse {
    pub bundle_override: BundleOverride,
}

#[derive(Debug, Serialize, Default)]
pub struct DeviceListParams {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub page: Option<String>,
}

impl DeviceListParams {
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut query_params = Vec::new();
        if let Some(limit) = self.limit {
            query_params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(ref order) = self.order {
            query_params.push(("order".to_string(), order.to_string()));
        }
        if let Some(ref page) = self.page {
            query_params.push(("page".to_string(), page.to_string()));
        }
        query_params
    }
}

#[derive(Debug, Serialize, Default)]
pub struct ListDevicesParams {
    pub prn: String,
    #[serde(flatten)]
    pub list: DeviceListParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub device_prn: String,
    pub inserted_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListDevicesResponse {
    pub devices: Vec<Device>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AddDeviceParams {
    pub prn: String,
    pub device_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddDeviceResponse {
    pub device: Device,
}

#[derive(Debug, Serialize)]
pub struct RemoveDeviceParams {
    pub prn: String,
    pub device_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveDeviceResponse {}

pub struct BundleOverridesApi<'a>(pub &'a Api);

impl<'a> BundleOverridesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateBundleOverrideParams,
    ) -> Result<Option<CreateBundleOverrideResponse>, Error> {
        self.0
            .execute(Method::POST, "/bundle_overrides", Some(json_body!(&params)))
            .await
    }

    pub async fn delete(
        &'a self,
        params: DeleteBundleOverrideParams,
    ) -> Result<Option<DeleteBundleOverrideResponse>, Error> {
        let bundle_override_prn: String = params.prn;
        self.0
            .execute(
                Method::DELETE,
                format!("/bundle_overrides/{bundle_override_prn}"),
                None,
            )
            .await
    }

    pub async fn get(
        &'a self,
        params: GetBundleOverrideParams,
    ) -> Result<Option<GetBundleOverrideResponse>, Error> {
        let bundle_override_prn: String = params.prn;
        self.0
            .execute(
                Method::GET,
                format!("/bundle_overrides/{bundle_override_prn}"),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListBundleOverridesParams,
    ) -> Result<Option<ListBundleOverridesResponse>, Error> {
        self.0
            .execute_with_params(
                Method::GET,
                "/bundle_overrides".to_string(),
                None,
                params.list.to_query_params(),
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateBundleOverrideParams,
    ) -> Result<Option<UpdateBundleOverrideResponse>, Error> {
        let bundle_override_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/bundle_overrides/{bundle_override_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn list_devices(
        &'a self,
        params: ListDevicesParams,
    ) -> Result<Option<ListDevicesResponse>, Error> {
        let bundle_override_prn: String = params.prn.clone();
        self.0
            .execute_with_params(
                Method::GET,
                format!("/bundle_overrides/{bundle_override_prn}/devices"),
                None,
                params.list.to_query_params(),
            )
            .await
    }

    pub async fn add_device(
        &'a self,
        params: AddDeviceParams,
    ) -> Result<Option<AddDeviceResponse>, Error> {
        let bundle_override_prn: String = params.prn.clone();
        let request_body = serde_json::json!({
            "device_prn": params.device_prn
        });

        self.0
            .execute(
                Method::POST,
                format!("/bundle_overrides/{bundle_override_prn}/devices"),
                Some(json_body!(&request_body)),
            )
            .await
    }

    pub async fn remove_device(
        &'a self,
        params: RemoveDeviceParams,
    ) -> Result<Option<RemoveDeviceResponse>, Error> {
        let bundle_override_prn: String = params.prn;
        let device_prn: String = params.device_prn;

        self.0
            .execute(
                Method::DELETE,
                format!("/bundle_overrides/{bundle_override_prn}/devices/{device_prn}"),
                None,
            )
            .await
    }
}
