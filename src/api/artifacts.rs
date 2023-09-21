use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Artifact {
    pub description: Option<String>,
    pub inserted_at: String,
    pub name: String,
    pub organization_prn: String,
    pub prn: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateArtifactParams {
    pub description: Option<String>,
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
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: String,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListArtifactsResponse {
    pub artifacts: Vec<Artifact>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateArtifactParams {
    pub prn: String,
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
        self.0
            .execute(Method::POST, "/artifacts", Some(json_body!(&params)))
            .await
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
        let search_string = params.search;
        self.0
            .execute(
                Method::GET,
                format!("/artifacts?search={search_string}"),
                None,
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateArtifactParams,
    ) -> Result<Option<UpdateArtifactResponse>, Error> {
        let artifact_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/artifacts/{artifact_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
