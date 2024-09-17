use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Bundle {
    pub artifact_versions: Vec<String>,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub updated_at: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateBundleParams {
    pub artifact_version_prns: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,
    pub organization_prn: String,
    pub name: Option<String>,
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
        let mut query_params = vec![("search".to_string(), params.search)];

        if let Some(limit) = params.limit {
            query_params.push(("limit".to_string(), limit.to_string()))
        }
        if let Some(order) = params.order {
            query_params.push(("order".to_string(), order))
        }

        if let Some(page) = params.page {
            query_params.push(("page".to_string(), page))
        }
        self.0
            .execute_with_params(Method::GET, "/bundles".to_string(), None, query_params)
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
