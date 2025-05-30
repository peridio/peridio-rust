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
}
