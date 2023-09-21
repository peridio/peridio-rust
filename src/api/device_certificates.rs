use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{json_body, Api};

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceCertificates {
    pub not_after: String,
    pub not_before: String,
    pub serial: String,
}

#[derive(Debug, Serialize)]
pub struct ListDeviceCertificateParams {
    pub organization_name: String,
    pub product_name: String,
    pub device_identifier: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListDeviceCertificateResponse {
    pub data: Vec<DeviceCertificates>,
}

#[derive(Debug, Serialize)]
pub struct GetDeviceCertificateParams {
    pub product_name: String,
    pub organization_name: String,
    pub certificate_serial: String,
    pub device_identifier: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetDeviceCertificateResponse {
    pub data: DeviceCertificates,
}

#[derive(Debug, Serialize)]
pub struct DeleteDeviceCertificateParams {
    pub product_name: String,
    pub organization_name: String,
    pub certificate_serial: String,
    pub device_identifier: String,
}

#[derive(Debug, Serialize)]
pub struct CreateDeviceCertificateParams {
    pub product_name: String,
    pub organization_name: String,
    pub device_identifier: String,
    pub cert: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDeviceCertificateResponse {
    pub data: DeviceCertificates,
}

pub struct DeviceCertificatesApi<'a>(pub &'a Api);

impl<'a> DeviceCertificatesApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateDeviceCertificateParams,
    ) -> Result<Option<CreateDeviceCertificateResponse>, Error> {
        let product_name = &params.product_name;
        let organization_name = &params.organization_name;
        let device_identifier = &params.device_identifier;

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}/certificates"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(
        &'a self,
        params: DeleteDeviceCertificateParams,
    ) -> Result<Option<()>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;
        let device_identifier = params.device_identifier;
        let certificate_serial = params.certificate_serial;

        self.0
            .execute(
                Method::DELETE,
                format!(
                    "/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}/certificates/{certificate_serial}"
                ),
                None,
            )
            .await
    }

    pub async fn get(
        &'a self,
        params: GetDeviceCertificateParams,
    ) -> Result<Option<GetDeviceCertificateResponse>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;
        let device_identifier = params.device_identifier;
        let certificate_serial = params.certificate_serial;

        self.0
            .execute(
                Method::GET,
                format!(
                    "/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}/certificates/{certificate_serial}"
                ),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListDeviceCertificateParams,
    ) -> Result<Option<ListDeviceCertificateResponse>, Error> {
        let organization_name = params.organization_name;
        let product_name = params.product_name;
        let device_identifier = params.device_identifier;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/products/{product_name}/devices/{device_identifier}/certificates"),
                None,
            )
            .await
    }
}
