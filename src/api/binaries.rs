use super::Error;

use crate::json_body;
use crate::Api;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use snafu::ResultExt;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BinaryState {
    Uploadable,
    Hashable,
    Hashing,
    Signable,
    Signed,
    Destroyed,
}

impl FromStr for BinaryState {
    type Err = Error;

    fn from_str(input: &str) -> Result<BinaryState, Self::Err> {
        match input {
            "uploadable" => Ok(BinaryState::Uploadable),
            "hashable" => Ok(BinaryState::Hashable),
            "hashing" => Ok(BinaryState::Hashing),
            "signable" => Ok(BinaryState::Signable),
            "signed" => Ok(BinaryState::Signed),
            "destroyed" => Ok(BinaryState::Destroyed),
            _ => Err(Error::Unknown {
                error: format!("given binary state '{input}' is not supported"),
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BinarySignature {
    pub signature: String,
    pub signing_key_prn: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Binary {
    pub artifact_version_prn: String,
    pub custom_metadata: Option<Map<String, Value>>,
    pub description: Option<String>,
    pub hash: Option<String>,
    pub organization_prn: String,
    pub prn: String,
    pub inserted_at: String,
    pub revision: u32,
    pub signatures: Option<Vec<BinarySignature>>,
    pub size: Option<u64>,
    pub state: BinaryState,
    pub target: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateBinaryParams {
    pub artifact_version_prn: String,
    pub custom_metadata: Option<Map<String, Value>>,
    pub description: Option<String>,
    pub hash: String,
    pub size: u64,
    pub target: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBinaryResponse {
    pub binary: Binary,
}

#[derive(Debug, Serialize)]
pub struct GetBinaryParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetBinaryResponse {
    pub binary: Binary,
}

#[derive(Debug, Serialize)]
pub struct ListBinariesParams {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: String,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListBinariesResponse {
    pub binaries: Vec<Binary>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateBinaryParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_metadata: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub state: Option<BinaryState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub size: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBinaryResponse {
    pub binary: Binary,
}

pub struct BinariesApi<'a>(pub &'a Api);

impl<'a> BinariesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateBinaryParams,
    ) -> Result<Option<CreateBinaryResponse>, Error> {
        self.0
            .execute(Method::POST, "/binaries", Some(json_body!(&params)))
            .await
    }

    pub async fn get(
        &'a self,
        params: GetBinaryParams,
    ) -> Result<Option<GetBinaryResponse>, Error> {
        let binary_prn: String = params.prn;
        self.0
            .execute(Method::GET, format!("/binaries/{binary_prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListBinariesParams,
    ) -> Result<Option<ListBinariesResponse>, Error> {
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
            .execute_with_params(Method::GET, "/binaries".to_string(), None, query_params)
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateBinaryParams,
    ) -> Result<Option<UpdateBinaryResponse>, Error> {
        let binary_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/binaries/{binary_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
