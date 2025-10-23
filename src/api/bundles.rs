use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{json_body, list_params::ListParams, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BundleBinary {
    pub custom_metadata: Option<Map<String, Value>>,
    pub prn: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bundle {
    V1(BundleV1),
    V2(BundleV2),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BundleV2 {
    pub binaries: Vec<BundleBinary>,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub updated_at: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BundleV1 {
    pub artifact_versions: Vec<String>,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub updated_at: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CreateBundleParams {
    V1(CreateBundleParamsV1),
    V2(CreateBundleParamsV2),
}

#[derive(Debug, Serialize)]
pub struct CreateBundleParamsV1 {
    pub artifact_version_prns: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateBundleParamsV2 {
    pub binaries: Vec<CreateBundleBinary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateBundleBinary {
    pub prn: String,
    pub custom_metadata: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBundleResponse {
    pub bundle: Bundle,
}

#[derive(Debug, Serialize)]
pub struct GetBundleParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetBundleResponse {
    pub bundle: Bundle,
}

#[derive(Debug, Serialize)]
pub struct DeleteBundleParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteBundleResponse {}

#[derive(Debug, Serialize)]
pub struct ListBundlesParams {
    #[serde(flatten)]
    pub list: ListParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListBundlesResponse {
    pub bundles: Vec<Bundle>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateBundleParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBundleResponse {
    pub bundle: Bundle,
}

pub struct BundlesApi<'a>(pub &'a Api);

impl<'a> BundlesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateBundleParams,
    ) -> Result<Option<CreateBundleResponse>, Error> {
        self.0
            .execute(Method::POST, "/bundles", Some(json_body!(&params)))
            .await
    }

    pub async fn delete(
        &'a self,
        params: DeleteBundleParams,
    ) -> Result<Option<DeleteBundleResponse>, Error> {
        let bundle_prn: String = params.prn;
        self.0
            .execute(Method::DELETE, format!("/bundles/{bundle_prn}"), None)
            .await
    }

    pub async fn get(
        &'a self,
        params: GetBundleParams,
    ) -> Result<Option<GetBundleResponse>, Error> {
        let bundle_prn: String = params.prn;
        self.0
            .execute(Method::GET, format!("/bundles/{bundle_prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListBundlesParams,
    ) -> Result<Option<ListBundlesResponse>, Error> {
        self.0
            .execute_with_params(
                Method::GET,
                "/bundles".to_string(),
                None,
                params.list.to_query_params(),
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateBundleParams,
    ) -> Result<Option<UpdateBundleResponse>, Error> {
        let bundle_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/bundles/{bundle_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
