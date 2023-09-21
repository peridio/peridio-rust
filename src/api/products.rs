use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ListProductParams {
    pub organization_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListProductResponse {
    pub data: Vec<Product>,
}

#[derive(Debug, Serialize)]
pub struct GetProductParams {
    pub product_name: String,
    pub organization_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetProductResponse {
    pub data: Product,
}

#[derive(Debug, Serialize)]
pub struct DeleteProductParams {
    pub product_name: String,
    pub organization_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateProductParams {
    pub organization_name: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProductResponse {
    pub data: Product,
}

#[derive(Debug, Serialize)]
pub struct UpdateProductParams {
    pub product_name: String,
    pub organization_name: String,
    pub product: UpdateProduct,
}

#[derive(Debug, Serialize)]
pub struct UpdateProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProductResponse {
    pub data: Product,
}

pub struct ProductApi<'a>(pub &'a Api);

impl<'a> ProductApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateProductParams,
    ) -> Result<Option<CreateProductResponse>, Error> {
        let organization_name = &params.organization_name;

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/products"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(&'a self, params: DeleteProductParams) -> Result<Option<()>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;

        self.0
            .execute(
                Method::DELETE,
                format!("/orgs/{organization_name}/products/{product_name}"),
                None,
            )
            .await
    }

    pub async fn get(
        &'a self,
        params: GetProductParams,
    ) -> Result<Option<GetProductResponse>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/products/{product_name}"),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListProductParams,
    ) -> Result<Option<ListProductResponse>, Error> {
        let organization_name = params.organization_name;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/products"),
                None,
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateProductParams,
    ) -> Result<Option<UpdateProductResponse>, Error> {
        let organization_name = &params.organization_name;
        let product_name = &params.product_name;

        self.0
            .execute(
                Method::PUT,
                format!("/orgs/{organization_name}/products/{product_name}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
