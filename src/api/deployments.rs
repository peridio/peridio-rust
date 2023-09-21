use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployment {
    pub conditions: DeploymentCondition,
    pub delta_updatable: bool,
    pub firmware_uuid: String,
    pub is_active: bool,
    pub name: String,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct DeploymentCondition {
    pub tags: Vec<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListDeploymentParams {
    pub organization_name: String,
    pub product_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListDeploymentResponse {
    pub data: Vec<Deployment>,
}

#[derive(Debug, Serialize)]
pub struct GetDeploymentParams {
    pub product_name: String,
    pub organization_name: String,
    pub deployment_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetDeploymentResponse {
    pub data: Deployment,
}

#[derive(Debug, Serialize)]
pub struct DeleteDeploymentParams {
    pub product_name: String,
    pub organization_name: String,
    pub deployment_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateDeploymentParams<'a> {
    pub product_name: String,
    pub organization_name: String,
    pub conditions: &'a DeploymentCondition,
    pub delta_updatable: bool,
    pub firmware: String,
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDeploymentResponse {
    pub data: Deployment,
}

#[derive(Debug, Serialize)]
pub struct UpdateDeploymentParams<'a> {
    pub product_name: String,
    pub organization_name: String,
    pub deployment_name: String,
    pub deployment: UpdateDeployment<'a>,
}

#[derive(Debug, Serialize)]
pub struct UpdateDeployment<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub delta_updatable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub conditions: Option<&'a UpdateDeploymentCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub firmware: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpdateDeploymentCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDeploymentResponse {
    pub data: Deployment,
}

pub struct DeploymentsApi<'a>(pub &'a Api);

impl<'a> DeploymentsApi<'a> {
    pub async fn create(
        &'a self,
        mut params: CreateDeploymentParams<'a>,
    ) -> Result<Option<CreateDeploymentResponse>, Error> {
        let product_name = &params.product_name;
        let organization_name = &params.organization_name;

        // must be false
        params.is_active = false;

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/products/{product_name}/deployments"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(&'a self, params: DeleteDeploymentParams) -> Result<Option<()>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;
        let deployment_name = params.deployment_name;

        self.0
            .execute(
                Method::DELETE,
                format!(
                    "/orgs/{organization_name}/products/{product_name}/deployments/{deployment_name}"
                ),
                None,
            )
            .await
    }

    pub async fn get(
        &'a self,
        params: GetDeploymentParams,
    ) -> Result<Option<GetDeploymentResponse>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;
        let deployment_name = params.deployment_name;

        self.0
            .execute(
                Method::GET,
                format!(
                    "/orgs/{organization_name}/products/{product_name}/deployments/{deployment_name}"
                ),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListDeploymentParams,
    ) -> Result<Option<ListDeploymentResponse>, Error> {
        let organization_name = params.organization_name;
        let product_name = params.product_name;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/products/{product_name}/deployments"),
                None,
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateDeploymentParams<'a>,
    ) -> Result<Option<UpdateDeploymentResponse>, Error> {
        let organization_name = &params.organization_name;
        let product_name = &params.product_name;
        let deployment_name = &params.deployment_name;

        self.0
            .execute(
                Method::PUT,
                format!("/orgs/{organization_name}/products/{product_name}/deployments/{deployment_name}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
