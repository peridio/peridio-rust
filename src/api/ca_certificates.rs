use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, list_params::ListParams, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct CaCertificate {
    pub description: Option<String>,
    pub not_after: String,
    pub not_before: String,
    pub serial: String,
    pub prn: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ListCaCertificateParams {
    #[serde(flatten)]
    pub list: ListParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListCaCertificateResponse {
    pub ca_certificates: Vec<CaCertificate>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetCaCertificateParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCaCertificateResponse {
    pub ca_certificate: CaCertificate,
}

#[derive(Debug, Serialize)]
pub struct DeleteCaCertificateParams {
    pub prn: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCaCertificateParams {
    pub certificate: String,
    pub verification_certificate: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCaCertificateResponse {
    pub ca_certificate: CaCertificate,
}

#[derive(Debug, Serialize)]
pub struct CreateVerificationCodeParams {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateVerificationCodeResponse {
    pub verification_code: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateCaCertificateParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCaCertificateResponse {
    pub ca_certificate: CaCertificate,
}

pub struct CaCertificatesApi<'a>(pub &'a Api);

impl<'a> CaCertificatesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateCaCertificateParams,
    ) -> Result<Option<CreateCaCertificateResponse>, Error> {
        self.0
            .execute(
                Method::POST,
                "/ca_certificates".to_string(),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn create_verification_code(
        &'a self,
        _params: CreateVerificationCodeParams,
    ) -> Result<Option<CreateVerificationCodeResponse>, Error> {
        self.0
            .execute(
                Method::POST,
                "/ca_certificates/verification_codes".to_string(),
                None,
            )
            .await
    }

    pub async fn delete(&'a self, params: DeleteCaCertificateParams) -> Result<Option<()>, Error> {
        let prn = params.prn;

        self.0
            .execute(Method::DELETE, format!("/ca_certificates/{prn}"), None)
            .await
    }

    pub async fn get(
        &'a self,
        params: GetCaCertificateParams,
    ) -> Result<Option<GetCaCertificateResponse>, Error> {
        let prn = params.prn;

        self.0
            .execute(Method::GET, format!("/ca_certificates/{prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListCaCertificateParams,
    ) -> Result<Option<ListCaCertificateResponse>, Error> {
        self.0
            .execute_with_params(
                Method::GET,
                "/ca_certificates".to_string(),
                None,
                params.list.to_query_params(),
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateCaCertificateParams,
    ) -> Result<Option<UpdateCaCertificateResponse>, Error> {
        let prn = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/ca_certificates/{prn}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
