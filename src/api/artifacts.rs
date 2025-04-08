use reqwest::Method;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use validator::Validate;

use crate::{json_body, list_params::ListParams, validators, Api};

use super::{Error, Validation};
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Artifact {
    pub custom_metadata: Option<Map<String, Value>>,
    pub description: Option<String>,
    pub inserted_at: String,
    pub name: String,
    pub organization_prn: String,
    pub prn: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Validate)]
pub struct CreateArtifactParams {
    #[validate(custom(function = "validators::validate_json_byte_length_1_000_000"))]
    pub custom_metadata: Option<Map<String, Value>>,
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    pub organization_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateArtifactResponse {
    pub artifact: Artifact,
}

#[derive(Debug, Serialize)]
pub struct GetArtifactParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetArtifactResponse {
    pub artifact: Artifact,
}

#[derive(Debug, Serialize)]
pub struct ListArtifactsParams {
    #[serde(flatten)]
    pub list: ListParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListArtifactsResponse {
    pub artifacts: Vec<Artifact>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize, Validate)]
pub struct UpdateArtifactParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[validate(custom(function = "validators::validate_json_byte_length_1_000_000"))]
    pub custom_metadata: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateArtifactResponse {
    pub artifact: Artifact,
}

pub struct ArtifactsApi<'a>(pub &'a Api);

impl<'a> ArtifactsApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateArtifactParams,
    ) -> Result<Option<CreateArtifactResponse>, Error> {
        match params.validate().context(Validation) {
            Ok(()) => {
                self.0
                    .execute(Method::POST, "/artifacts", Some(json_body!(&params)))
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get(
        &'a self,
        params: GetArtifactParams,
    ) -> Result<Option<GetArtifactResponse>, Error> {
        let artifact_prn: String = params.prn;
        self.0
            .execute(Method::GET, format!("/artifacts/{artifact_prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListArtifactsParams,
    ) -> Result<Option<ListArtifactsResponse>, Error> {
        self.0
            .execute_with_params(
                Method::GET,
                "/artifacts".to_string(),
                None,
                params.list.to_query_params(),
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateArtifactParams,
    ) -> Result<Option<UpdateArtifactResponse>, Error> {
        let artifact_prn: &String = &params.prn;

        match params.validate().context(Validation) {
            Ok(()) => {
                self.0
                    .execute(
                        Method::PATCH,
                        format!("/artifacts/{artifact_prn}"),
                        Some(json_body!(&params)),
                    )
                    .await
            }
            Err(err) => Err(err),
        }
    }
}
