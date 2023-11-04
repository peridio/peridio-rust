use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct SigningKey {
    pub value: Option<String>,
    pub inserted_at: String,
    pub name: String,
    pub organization_prn: String,
    pub prn: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSigningKeyParams {
    pub value: String,
    pub name: String,
    pub organization_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSigningKeyResponse {
    pub signing_key: SigningKey,
}

#[derive(Debug, Serialize)]
pub struct GetSigningKeyParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSigningKeyResponse {
    pub signing_key: SigningKey,
}

#[derive(Debug, Serialize)]
pub struct ListSigningKeysParams {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: String,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListSigningKeysResponse {
    pub signing_keys: Vec<SigningKey>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateSigningKeyParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSigningKeyResponse {
    pub signing_key: SigningKey,
}

#[derive(Debug, Serialize)]
pub struct DeleteSigningKeyParams {
    pub signing_key_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteSigningKeyResponse {}

pub struct SigningKeysApi<'a>(pub &'a Api);

impl<'a> SigningKeysApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateSigningKeyParams,
    ) -> Result<Option<CreateSigningKeyResponse>, Error> {
        self.0
            .execute(Method::POST, "/signing_keys", Some(json_body!(&params)))
            .await
    }

    pub async fn get(
        &'a self,
        params: GetSigningKeyParams,
    ) -> Result<Option<GetSigningKeyResponse>, Error> {
        let signing_key_prn: String = params.prn;
        self.0
            .execute(
                Method::GET,
                format!("/signing_keys/{signing_key_prn}"),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListSigningKeysParams,
    ) -> Result<Option<ListSigningKeysResponse>, Error> {
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
            .execute_with_params(Method::GET, "/signing_keys".to_string(), None, query_params)
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateSigningKeyParams,
    ) -> Result<Option<UpdateSigningKeyResponse>, Error> {
        let signing_key_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/signing_keys/{signing_key_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(
        &'a self,
        params: DeleteSigningKeyParams,
    ) -> Result<Option<DeleteSigningKeyResponse>, Error> {
        let signing_key_prn: String = params.signing_key_prn;
        self.0
            .execute(
                Method::DELETE,
                format!("/signing_keys/{signing_key_prn}"),
                None,
            )
            .await
    }
}
