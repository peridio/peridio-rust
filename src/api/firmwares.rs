use std::path::Path;

use reqwest::{Body, Method};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{multipart_body, Api};

use super::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Firmware {
    pub architecture: String,
    pub author: Option<String>,
    pub inserted_at: String,
    pub platform: String,
    pub product: String,
    pub updated_at: String,
    pub uuid: String,
    pub vcs_identifier: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct ListFirmwareParams {
    pub organization_name: String,
    pub product_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListFirmwareResponse {
    pub data: Vec<Firmware>,
}

#[derive(Debug, Serialize)]
pub struct GetFirmwareParams {
    pub product_name: String,
    pub organization_name: String,
    pub firmware_uuid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetFirmwareResponse {
    pub data: Firmware,
}

#[derive(Debug, Serialize)]
pub struct DeleteFirmwareParams {
    pub product_name: String,
    pub organization_name: String,
    pub firmware_uuid: String,
}

#[derive(Debug, Serialize)]
pub struct CreateFirmwareParams {
    pub product_name: String,
    pub organization_name: String,
    pub firmware_path: String,
    pub ttl: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFirmwareResponse {
    pub data: Firmware,
}

pub struct FirmwaresApi<'a>(pub &'a Api);

impl<'a> FirmwaresApi<'a> {
    fn get_firmware_file_name(path: &str) -> String {
        String::from(Path::new(path).file_name().unwrap().to_str().unwrap())
    }

    pub async fn create(
        &'a self,
        params: CreateFirmwareParams,
    ) -> Result<Option<CreateFirmwareResponse>, Error> {
        let product_name = &params.product_name;
        let organization_name = &params.organization_name;
        let firmware_path = &params.firmware_path;
        let ttl = &params.ttl;

        let form = reqwest::multipart::Form::new();

        let file = File::open(firmware_path).await.unwrap();
        let file_name = Self::get_firmware_file_name(firmware_path);

        let stream = FramedRead::new(file, BytesCodec::new());

        let stream_part = reqwest::multipart::Part::stream(Body::wrap_stream(stream))
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .unwrap();

        let form = form.part("firmware", stream_part);

        let form = if let Some(ttl_value) = ttl {
            form.text("ttl", ttl_value.to_string())
        } else {
            form
        };

        self.0
            .execute(
                Method::POST,
                format!("/orgs/{organization_name}/products/{product_name}/firmwares"),
                Some(multipart_body!(form)),
            )
            .await
    }

    pub async fn delete(&'a self, params: DeleteFirmwareParams) -> Result<Option<()>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;
        let firmware_uuid = params.firmware_uuid;

        self.0
            .execute(
                Method::DELETE,
                format!(
                    "/orgs/{organization_name}/products/{product_name}/firmwares/{firmware_uuid}"
                ),
                None,
            )
            .await
    }

    pub async fn get(
        &'a self,
        params: GetFirmwareParams,
    ) -> Result<Option<GetFirmwareResponse>, Error> {
        let product_name = params.product_name;
        let organization_name = params.organization_name;
        let firmware_uuid = params.firmware_uuid;

        self.0
            .execute(
                Method::GET,
                format!(
                    "/orgs/{organization_name}/products/{product_name}/firmwares/{firmware_uuid}"
                ),
                None,
            )
            .await
    }

    pub async fn list(
        &'a self,
        params: ListFirmwareParams,
    ) -> Result<Option<ListFirmwareResponse>, Error> {
        let organization_name = params.organization_name;
        let product_name = params.product_name;

        self.0
            .execute(
                Method::GET,
                format!("/orgs/{organization_name}/products/{product_name}/firmwares"),
                None,
            )
            .await
    }
}
