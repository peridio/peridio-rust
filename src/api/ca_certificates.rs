use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct CaCertificate {
    pub description: Option<String>,
    pub not_after: String,
    pub not_before: String,
    pub serial: String,
    pub jitp: Option<CaCertificateJitp>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CaCertificateJitp {
    pub description: String,
    pub tags: Vec<String>,
    pub product_name: String,
    pub cohort_prn: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListCaCertificateParams {
    pub organization_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListCaCertificateResponse {
    pub data: Vec<CaCertificate>,
}

#[derive(Debug, Serialize)]
pub struct GetCaCertificateParams {
    pub organization_name: String,
    pub ca_certificate_serial: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCaCertificateResponse {
    pub data: CaCertificate,
}

#[derive(Debug, Serialize)]
pub struct DeleteCaCertificateParams {
    pub organization_name: String,
    pub ca_certificate_serial: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCaCertificateParams {
    pub organization_name: String,
    pub certificate: String,
    pub verification_certificate: String,
    pub description: Option<String>,
    pub jitp: Option<CaCertificateJitp>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCaCertificateResponse {
    pub data: CaCertificate,
}

#[derive(Debug, Serialize)]
pub struct CreateVerificationCodeParams {
    pub organization_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateVerificationCodeResponse {
    pub data: VerificationCode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerificationCode {
    pub verification_code: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateCaCertificateParams {
    pub organization_name: String,
    pub ca_certificate_serial: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub jitp: Option<Option<CaCertificateJitp>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCaCertificateResponse {
    pub data: CaCertificate,
}

pub struct CaCertificatesApi<'a>(pub &'a Api);

impl<'a> CaCertificatesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateCaCertificateParams,
    ) -> Result<Option<CreateCaCertificateResponse>, Error> {
        let organization_name = &params.organization_name;

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/ca_certificates"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn create_verification_code(
        &'a self,
        params: CreateVerificationCodeParams,
    ) -> Result<Option<CreateVerificationCodeResponse>, Error> {
        let organization_name = &params.organization_name;

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/ca_certificates/verification_codes"),
                None,
            )
            .await
    }

    pub async fn delete(&'a self, params: DeleteCaCertificateParams) -> Result<Option<()>, Error> {
        let organization_name = params.organization_name;
        let ca_certificate_serial = params.ca_certificate_serial;

        self.0
            .execute(
                Method::DELETE,
                format!("/orgs/{organization_name}/ca_certificates/{ca_certificate_serial}"),
                None,
            )
            .await
    }

    pub async fn get(
        &'a self,
        params: GetCaCertificateParams,
    ) -> Result<Option<GetCaCertificateResponse>, Error> {
        let organization_name = params.organization_name;
        let ca_certificate_serial = params.ca_certificate_serial;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/ca_certificates/{ca_certificate_serial}"),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListCaCertificateParams,
    ) -> Result<Option<ListCaCertificateResponse>, Error> {
        let organization_name = params.organization_name;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/ca_certificates"),
                None,
            )
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateCaCertificateParams,
    ) -> Result<Option<UpdateCaCertificateResponse>, Error> {
        let organization_name = &params.organization_name;
        let ca_certificate_serial = &params.ca_certificate_serial;

        self.0
            .execute(
                Method::PUT,
                format!("/orgs/{organization_name}/ca_certificates/{ca_certificate_serial}"),
                Some(json_body!(&params)),
            )
            .await
    }
}
