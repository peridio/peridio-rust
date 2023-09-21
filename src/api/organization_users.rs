use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrganizationUser {
    pub email: String,
    pub role: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct ListOrganizationUserParams {
    pub organization_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListOrganizationUserResponse {
    pub data: Vec<OrganizationUser>,
}

#[derive(Debug, Serialize)]
pub struct GetOrganizationUserParams {
    pub organization_name: String,
    pub user_username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetOrganizationUserResponse {
    pub data: OrganizationUser,
}

#[derive(Debug, Serialize)]
pub struct RemoveOrganizationUserParams {
    pub organization_name: String,
    pub user_username: String,
}

#[derive(Debug, Serialize)]
pub struct AddOrganizationUserParams {
    pub organization_name: String,
    pub role: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddOrganizationUserResponse {
    pub data: OrganizationUser,
}

#[derive(Debug, Serialize)]
pub struct UpdateOrganizationUserParams {
    pub organization_name: String,
    pub user_username: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateOrganizationUserResponse {
    pub data: OrganizationUser,
}

pub struct OrganizationUsersApi<'a>(pub &'a Api);

impl<'a> OrganizationUsersApi<'a> {
    pub async fn add(
        &'a self,
        params: AddOrganizationUserParams,
    ) -> Result<Option<AddOrganizationUserResponse>, Error> {
        let organization_name = &params.organization_name;

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/users"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn remove(
        &'a self,
        params: RemoveOrganizationUserParams,
    ) -> Result<Option<()>, Error> {
        let organization_name = params.organization_name;
        let user_username = params.user_username;

        self.0
            .execute(
                Method::DELETE,
                format!("/orgs/{organization_name}/users/{user_username}"),
                None,
            )
            .await
    }

    pub async fn get(
        &'a self,
        params: GetOrganizationUserParams,
    ) -> Result<Option<GetOrganizationUserResponse>, Error> {
        let organization_name = params.organization_name;
        let user_username = params.user_username;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/users/{user_username}"),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListOrganizationUserParams,
    ) -> Result<Option<ListOrganizationUserResponse>, Error> {
        let organization_name = params.organization_name;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/users"),
                None,
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateOrganizationUserParams,
    ) -> Result<Option<UpdateOrganizationUserResponse>, Error> {
        let organization_name = &params.organization_name;
        let user_username = &params.user_username;

        self.0
            .execute(
                Method::PUT,
                format!("/orgs/{organization_name}/users/{user_username}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
