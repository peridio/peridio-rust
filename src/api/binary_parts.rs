use reqwest::Method;

use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct BinaryPart {
    pub binary_prn: Option<String>,
    pub hash: String,
    pub index: u16,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub size: u64,
    pub state: String,
    pub updated_at: String,
    pub presigned_upload_url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateBinaryPartParams {
    pub binary_prn: String,
    pub index: u16,
    pub expected_binary_size: u64,
    pub hash: String,
    pub size: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBinaryPartResponse {
    pub binary_part: BinaryPart,
}

#[derive(Debug, Serialize)]
pub struct ListBinaryPartsParams {
    pub binary_prn: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListBinaryPart {
    pub binary_prn: Option<String>,
    pub hash: String,
    pub index: u16,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub size: u64,
    pub state: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListBinaryPartsResponse {
    pub binary_parts: Vec<ListBinaryPart>,
    pub next_page: Option<String>,
}

pub struct BinaryPartsApi<'a>(pub &'a Api);

impl<'a> BinaryPartsApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateBinaryPartParams,
    ) -> Result<Option<CreateBinaryPartResponse>, Error> {
        let part_number = params.index as u64;
        let index = part_number - 1;

        self.0
            .execute_with_headers(
                Method::PUT,
                format!("/binaries/{}/parts/{}", params.binary_prn, params.index),
                Some(json_body!(&params)),
                vec![(
                    "content-range".to_string(),
                    format!(
                        "bytes {}-{}/{}",
                        // size is the total length, here is 0 indexed so we subtract 1
                        // if index = 0, start = 0 * (size - 1) = 0, end = (size - 1) * (0 + 1) = (size - 1) = total size zero indexed
                        index * params.size,
                        (params.size * part_number) - 1,
                        params.expected_binary_size
                    ),
                )],
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListBinaryPartsParams,
    ) -> Result<Option<ListBinaryPartsResponse>, Error> {
        self.0
            .execute(
                Method::GET,
                format!("/binaries/{}/parts", params.binary_prn),
                None,
            )
            .await
    }
}
