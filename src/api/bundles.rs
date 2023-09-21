use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Bundle {
    pub artifact_versions: Vec<BundleArtifactVersion>,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BundleArtifactVersion {
    pub prn: String,
    pub index: u16,
}

#[derive(Debug, Serialize)]
pub struct CreateBundleParams {
    pub artifact_version_prns: Vec<String>,
    pub organization_prn: String,
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
pub struct ListBundlesParams {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: String,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListBundlesResponse {
    pub bundles: Vec<Bundle>,
    pub next_page: Option<String>,
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
        let search_string = params.search;
        self.0
            .execute(
                Method::GET,
                format!("/bundles?search={search_string}"),
                None,
            )
            .await
    }
}
