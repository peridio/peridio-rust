use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Cohort {
    pub description: Option<String>,
    pub name: String,
    pub organization_prn: String,
    pub product_prn: String,
    pub prn: String,
    pub inserted_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCohortParams {
    pub description: Option<String>,
    pub name: String,
    pub organization_prn: String,
    pub product_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCohortResponse {
    pub cohort: Cohort,
}

#[derive(Debug, Serialize)]
pub struct GetCohortParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCohortResponse {
    pub cohort: Cohort,
}

#[derive(Debug, Serialize)]
pub struct ListCohortsParams {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: String,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListCohortsResponse {
    pub cohorts: Vec<Cohort>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateCohortParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCohortResponse {
    pub cohort: Cohort,
}

pub struct CohortsApi<'a>(pub &'a Api);

impl<'a> CohortsApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateCohortParams,
    ) -> Result<Option<CreateCohortResponse>, Error> {
        self.0
            .execute(Method::POST, "/cohorts", Some(json_body!(&params)))
            .await
    }

    pub async fn get(
        &'a self,
        params: GetCohortParams,
    ) -> Result<Option<GetCohortResponse>, Error> {
        let cohort_prn: String = params.prn;
        self.0
            .execute(Method::GET, format!("/cohorts/{cohort_prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListCohortsParams,
    ) -> Result<Option<ListCohortsResponse>, Error> {
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
            .execute_with_params(Method::GET, "/cohorts".to_string(), None, query_params)
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateCohortParams,
    ) -> Result<Option<UpdateCohortResponse>, Error> {
        let cohort_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/cohorts/{cohort_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
