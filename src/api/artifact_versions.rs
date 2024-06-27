use reqwest::Method;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use validator::Validate;

use crate::{json_body, validators, Api};

use super::{Error, Validation};
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtifactVersion {
    pub artifact_prn: String,
    pub custom_metadata: Option<Map<String, Value>>,
    pub description: Option<String>,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub version: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Validate)]
pub struct CreateArtifactVersionParams {
    pub artifact_prn: String,
    #[validate(custom(function = "validators::validate_json_byte_length_1_000_000"))]
    pub custom_metadata: Option<Map<String, Value>>,
    pub description: Option<String>,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateArtifactVersionResponse {
    pub artifact_version: ArtifactVersion,
}

#[derive(Debug, Serialize)]
pub struct GetArtifactVersionParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetArtifactVersionResponse {
    pub artifact_version: ArtifactVersion,
}

#[derive(Debug, Serialize)]
pub struct ListArtifactVersionsParams {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: String,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListArtifactVersionsResponse {
    pub artifact_versions: Vec<ArtifactVersion>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize, Validate)]
pub struct UpdateArtifactVersionParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[validate(custom(function = "validators::validate_json_byte_length_1_000_000"))]
    pub custom_metadata: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateArtifactVersionResponse {
    pub artifact_version: ArtifactVersion,
}

pub struct ArtifactVersionsApi<'a>(pub &'a Api);

impl<'a> ArtifactVersionsApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateArtifactVersionParams,
    ) -> Result<Option<CreateArtifactVersionResponse>, Error> {
        match params.validate().context(Validation) {
            Ok(()) => {
                self.0
                    .execute(
                        Method::POST,
                        "/artifact_versions",
                        Some(json_body!(&params)),
                    )
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get(
        &'a self,
        params: GetArtifactVersionParams,
    ) -> Result<Option<GetArtifactVersionResponse>, Error> {
        let artifact_version_prn: String = params.prn;
        self.0
            .execute(
                Method::GET,
                format!("/artifact_versions/{artifact_version_prn}"),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListArtifactVersionsParams,
    ) -> Result<Option<ListArtifactVersionsResponse>, Error> {
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
            .execute_with_params(
                Method::GET,
                "/artifact_versions".to_string(),
                None,
                query_params,
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateArtifactVersionParams,
    ) -> Result<Option<UpdateArtifactVersionResponse>, Error> {
        let artifact_version_prn: &String = &params.prn;

        match params.validate().context(Validation) {
            Ok(()) => {
                self.0
                    .execute(
                        Method::PATCH,
                        format!("/artifact_versions/{artifact_version_prn}"),
                        Some(json_body!(&params)),
                    )
                    .await
            }
            Err(err) => Err(err),
        }
    }
}
