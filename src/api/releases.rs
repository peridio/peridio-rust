use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    pub bundle_prn: String,
    pub cohort_prn: String,
    pub description: Option<String>,
    pub inserted_at: String,
    pub name: String,
    pub next_release_prn: Option<String>,
    pub organization_prn: String,
    pub phase_type: String,
    pub phase_value: f64,
    pub required: bool,
    pub schedule_date: String,
    pub schedule_complete: bool,
    pub prn: String,
    pub updated_at: String,
    pub version: Option<String>,
    pub version_requirement: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateReleaseParams {
    pub bundle_prn: String,
    pub cohort_prn: String,
    pub description: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub next_release_prn: Option<String>,
    pub organization_prn: String,
    pub phase_value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub previous_release_prn: Option<String>,
    pub required: bool,
    pub schedule_date: String,
    pub version: Option<String>,
    pub version_requirement: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateReleaseResponse {
    pub release: Release,
}

#[derive(Debug, Serialize)]
pub struct GetReleaseParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetReleaseResponse {
    pub release: Release,
}

#[derive(Debug, Serialize)]
pub struct ListReleasesParams {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: String,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListReleasesResponse {
    pub releases: Vec<Release>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateReleaseParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub next_release_prn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub phase_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub schedule_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub version_requirement: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateReleaseResponse {
    pub release: Release,
}

pub struct ReleasesApi<'a>(pub &'a Api);

impl<'a> ReleasesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateReleaseParams,
    ) -> Result<Option<CreateReleaseResponse>, Error> {
        self.0
            .execute(Method::POST, "/releases", Some(json_body!(&params)))
            .await
    }

    pub async fn get(
        &'a self,
        params: GetReleaseParams,
    ) -> Result<Option<GetReleaseResponse>, Error> {
        let release_prn: String = params.prn;
        self.0
            .execute(Method::GET, format!("/releases/{release_prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListReleasesParams,
    ) -> Result<Option<ListReleasesResponse>, Error> {
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
            .execute_with_params(Method::GET, "/releases".to_string(), None, query_params)
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateReleaseParams,
    ) -> Result<Option<UpdateReleaseResponse>, Error> {
        let release_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/releases/{release_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
