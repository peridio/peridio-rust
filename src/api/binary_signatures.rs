use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct BinarySignature {
    pub binary_prn: String,
    pub inserted_at: String,
    pub organization_prn: String,
    pub prn: String,
    pub signature: String,
    pub signing_key_prn: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateBinarySignatureParams {
    pub binary_prn: String,
    pub signing_key_prn: String,
    pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBinarySignatureResponse {
    pub binary_signature: BinarySignature,
}

#[derive(Debug, Serialize)]
pub struct DeleteBinarySignatureParams {
    pub binary_signature_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteBinarySignatureResponse {}

pub struct BinarySignaturesApi<'a>(pub &'a Api);

impl<'a> BinarySignaturesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateBinarySignatureParams,
    ) -> Result<Option<CreateBinarySignatureResponse>, Error> {
        self.0
            .execute(
                Method::POST,
                "/binary_signatures",
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(
        &'a self,
        params: DeleteBinarySignatureParams,
    ) -> Result<Option<DeleteBinarySignatureResponse>, Error> {
        let binary_signature_prn: String = params.binary_signature_prn;
        self.0
            .execute(
                Method::DELETE,
                format!("/binary_signatures/{binary_signature_prn}"),
                None,
            )
            .await
    }
}
