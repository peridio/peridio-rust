use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductUser {
    pub email: String,
    pub role: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct ListProductUserParams {
    pub product_name: String,
    pub organization_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListProductUserResponse {
    pub data: Vec<ProductUser>,
}

#[derive(Debug, Serialize)]
pub struct GetProductUserParams {
    pub product_name: String,
    pub organization_name: String,
    pub user_username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetProductUserResponse {
    pub data: ProductUser,
}

#[derive(Debug, Serialize)]
pub struct RemoveProductUserParams {
    pub product_name: String,
    pub organization_name: String,
    pub user_username: String,
}

#[derive(Debug, Serialize)]
pub struct AddProductUserParams {
    pub product_name: String,
    pub organization_name: String,
    pub role: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddProductUserResponse {
    pub data: ProductUser,
}

#[derive(Debug, Serialize)]
pub struct UpdateProductUserParams {
    pub product_name: String,
    pub organization_name: String,
    pub user_username: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProductUserResponse {
    pub data: ProductUser,
}

pub struct ProductUsersApi<'a>(pub &'a Api);

impl<'a> ProductUsersApi<'a> {
    pub async fn add(
        &'a self,
        params: AddProductUserParams,
    ) -> Result<Option<AddProductUserResponse>, Error> {
        let product_name = &params.product_name;
        let organization_name = &params.organization_name;

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/products/{product_name}/users"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn remove(&'a self, params: RemoveProductUserParams) -> Result<Option<()>, Error> {
        let product_name = &params.product_name;
        let organization_name = params.organization_name;
        let user_username = params.user_username;

        self.0
            .execute(
                Method::DELETE,
                format!("/orgs/{organization_name}/products/{product_name}/users/{user_username}"),
                None,
            )
            .await
    }

    pub async fn get(
        &'a self,
        params: GetProductUserParams,
    ) -> Result<Option<GetProductUserResponse>, Error> {
        let product_name = &params.product_name;
        let organization_name = params.organization_name;
        let user_username = params.user_username;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/products/{product_name}/users/{user_username}"),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListProductUserParams,
    ) -> Result<Option<ListProductUserResponse>, Error> {
        let product_name = &params.product_name;
        let organization_name = params.organization_name;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/products/{product_name}/users"),
                None,
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateProductUserParams,
    ) -> Result<Option<UpdateProductUserResponse>, Error> {
        let product_name = &params.product_name;
        let organization_name = &params.organization_name;
        let user_username = &params.user_username;

        self.0
            .execute(
                Method::PUT,
                format!("/orgs/{organization_name}/products/{product_name}/users/{user_username}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
