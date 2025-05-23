use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, list_params::ListParams, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceCertificate {
    pub not_after: String,
    pub not_before: String,
    pub prn: String,
    pub serial: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ListDeviceCertificateParams {
    #[serde(flatten)]
    pub list: ListParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListDeviceCertificateResponse {
    pub device_certificates: Vec<DeviceCertificate>,
    pub next_page: String,
}

#[derive(Debug, Serialize)]
pub struct GetDeviceCertificateParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetDeviceCertificateResponse {
    pub device_certificate: DeviceCertificate,
}

#[derive(Debug, Serialize)]
pub struct DeleteDeviceCertificateParams {
    pub prn: String,
}

#[derive(Debug, Serialize)]
pub struct CreateDeviceCertificateParams {
    pub certificate: String,
    pub device_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDeviceCertificateResponse {
    pub device_certificate: DeviceCertificate,
}

pub struct DeviceCertificatesApi<'a>(pub &'a Api);

impl<'a> DeviceCertificatesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateDeviceCertificateParams,
    ) -> Result<Option<CreateDeviceCertificateResponse>, Error> {
        self.0
            .execute(
                Method::POST,
                "/device_certificates".to_string(),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(
        &'a self,
        params: DeleteDeviceCertificateParams,
    ) -> Result<Option<()>, Error> {
        let prn = params.prn;

        self.0
            .execute(Method::DELETE, format!("/device_certificates/{prn}"), None)
            .await
    }

    pub async fn get(
        &'a self,
        params: GetDeviceCertificateParams,
    ) -> Result<Option<GetDeviceCertificateResponse>, Error> {
        let prn = params.prn;

        self.0
            .execute(Method::GET, format!("/device_certificates/{prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListDeviceCertificateParams,
    ) -> Result<Option<ListDeviceCertificateResponse>, Error> {
        self.0
            .execute_with_params(
                Method::GET,
                "/device_certificates".to_string(),
                None,
                params.list.to_query_params(),
            )
            .await
    }
}
