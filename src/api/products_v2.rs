use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductV2 {
    pub inserted_at: String,
    pub name: String,
    pub organization_prn: String,
    pub prn: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateProductV2Params {
    pub name: String,
    pub organization_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProductV2Response {
    pub product: ProductV2,
}

#[derive(Debug, Serialize)]
pub struct GetProductV2Params {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetProductV2Response {
    pub product: ProductV2,
}

#[derive(Debug, Serialize)]
pub struct ListProductsV2Params {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: String,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListProductsV2Response {
    pub products: Vec<ProductV2>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateProductV2Params {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProductV2Response {
    pub product: ProductV2,
}

pub struct DeleteProductV2Params {
    pub product_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteProductV2Response {}

pub struct ProductsV2Api<'a>(pub &'a Api);

impl<'a> ProductsV2Api<'a> {
    pub async fn create(
        &'a self,
        params: CreateProductV2Params,
    ) -> Result<Option<CreateProductV2Response>, Error> {
        self.0
            .execute(Method::POST, "/products", Some(json_body!(&params)))
            .await
    }

    pub async fn get(
        &'a self,
        params: GetProductV2Params,
    ) -> Result<Option<GetProductV2Response>, Error> {
        let product_prn: String = params.prn;
        self.0
            .execute(Method::GET, format!("/products/{product_prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListProductsV2Params,
    ) -> Result<Option<ListProductsV2Response>, Error> {
        let search_string = params.search;
        self.0
            .execute(
                Method::GET,
                format!("/products?search={search_string}"),
                None,
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateProductV2Params,
    ) -> Result<Option<UpdateProductV2Response>, Error> {
        let product_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/products/{product_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(
        &'a self,
        params: DeleteProductV2Params,
    ) -> Result<Option<DeleteProductV2Response>, Error> {
        let product_prn: String = params.product_prn;
        self.0
            .execute(Method::DELETE, format!("/products/{product_prn}"), None)
            .await
    }
}
