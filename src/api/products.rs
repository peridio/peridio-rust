use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, list_params::ListParams, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub archived: bool,
    pub inserted_at: String,
    pub name: String,
    pub prn: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateProductParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub archived: Option<bool>,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProductResponse {
    pub product: Product,
}

#[derive(Debug, Serialize)]
pub struct GetProductParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetProductResponse {
    pub product: Product,
}

#[derive(Debug, Serialize)]
pub struct ListProductsParams {
    #[serde(flatten)]
    pub list: ListParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListProductsResponse {
    pub products: Vec<Product>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateProductParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub archived: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProductResponse {
    pub product: Product,
}

pub struct ProductsApi<'a>(pub &'a Api);

impl<'a> ProductsApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateProductParams,
    ) -> Result<Option<CreateProductResponse>, Error> {
        self.0
            .execute(Method::POST, "/products", Some(json_body!(&params)))
            .await
    }

    pub async fn get(
        &'a self,
        params: GetProductParams,
    ) -> Result<Option<GetProductResponse>, Error> {
        let product_prn: String = params.prn;
        self.0
            .execute(Method::GET, format!("/products/{product_prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListProductsParams,
    ) -> Result<Option<ListProductsResponse>, Error> {
        self.0
            .execute_with_params(
                Method::GET,
                "/products".to_string(),
                None,
                params.list.to_query_params(),
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateProductParams,
    ) -> Result<Option<UpdateProductResponse>, Error> {
        let product_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/products/{product_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
