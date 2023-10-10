mod users;

pub mod artifact_versions;
pub mod artifacts;
pub mod binaries;
pub mod binary_parts;
pub mod binary_signatures;
pub mod bundles;
pub mod ca_certificates;
pub mod cohorts;
pub mod deployments;
pub mod device_certificates;
pub mod devices;
pub mod error;
pub mod firmwares;
pub mod organization_users;
pub mod product_users;
pub mod products;
pub mod products_v2;
pub mod releases;
pub mod signing_keys;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{header, Client, ClientBuilder, Method};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub use artifacts::ArtifactsApi;
pub use binaries::BinariesApi;
pub use binary_parts::BinaryPartsApi;
pub use binary_signatures::BinarySignaturesApi;
pub use ca_certificates::CaCertificatesApi;
pub use cohorts::CohortsApi;
pub use deployments::DeploymentsApi;
pub use device_certificates::DeviceCertificatesApi;
pub use devices::DevicesApi;
pub use firmwares::FirmwaresApi;
pub use organization_users::OrganizationUsersApi;
pub use product_users::ProductUsersApi;
pub use products::ProductApi;
pub use products_v2::ProductsV2Api;
pub use releases::ReleasesApi;
pub use reqwest::Body;
pub use signing_keys::SigningKeysApi;
pub use users::UsersApi;

use self::artifact_versions::ArtifactVersionsApi;
use self::bundles::BundlesApi;

const CERT_ADMIN_API: &[u8] = include_bytes!("../../certificates/admin-api.pem");
const CERT_NERVESHUB: &[u8] = include_bytes!("../../certificates/nerveshub.pem");
const CERT_PERIDIO: &[u8] = include_bytes!("../../certificates/peridio.pem");
const LATEST_ENDPOINT: &str = "https://api.cremini.peridio.com";

type ContentType = &'static str;

enum BodyType {
    Body((ContentType, Body)),
    Multipart(reqwest::multipart::Form),
}

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    #[snafu(display("Api request failed with error: {}", source))]
    RequestFailed { source: reqwest::Error },

    #[snafu(display("Bad response {}", source))]
    BadResponse { source: reqwest::Error },

    #[snafu(display("Bad request {}", source))]
    BadRequestParams { source: reqwest::Error },

    #[snafu(display("JSON serialization failed {}", source))]
    JsonSerializationFailed { source: serde_json::Error },

    #[snafu(display("{}", error))]
    Unknown { error: String },

    #[snafu(display("{}", error))]
    Unauthorized { error: UnauthorizedError },

    #[snafu(display("{}", error))]
    NotFound { error: NotFoundError },

    #[snafu(display("{}", error))]
    InternalServer { error: InternalServerError },

    #[snafu(display("{}", error))]
    Conflict { error: ConflictError },

    #[snafu(display("{}", error))]
    UnprocessableEntity { error: UnprocessableEntityError },
}

#[macro_export]
macro_rules! json_body {
    ($v:expr) => {
        $crate::api::BodyType::Body((
            "application/json",
            serde_json::to_vec($v)
                .context(super::JsonSerializationFailed)?
                .into(),
        ))
    };
}

#[macro_export]
macro_rules! multipart_body {
    ($v:expr) => {
        $crate::api::BodyType::Multipart(($v.into()))
    };
}

#[derive(Debug, Clone)]
pub struct Api {
    api_key: String,
    endpoint: String,
    http: Client,
}

pub struct ApiOptions {
    pub api_key: String,
    pub endpoint: Option<String>,
    pub ca_bundle_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnauthorizedError {
    status: String,
}

impl fmt::Display for UnauthorizedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let j = serde_json::to_string(&self).unwrap();
        write!(f, "{j}")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotFoundError {
    errors: NotFoundErrors,
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let j = serde_json::to_string(&self).unwrap();
        write!(f, "{j}")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotFoundErrors {
    detail: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InternalServerError {
    errors: serde_json::Value,
}

impl fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let j = serde_json::to_string(&self).unwrap();
        write!(f, "{j}")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConflictError {
    errors: HashMap<String, Vec<String>>,
}

impl fmt::Display for ConflictError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let j = serde_json::to_string(&self).unwrap();
        write!(f, "{j}")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnprocessableEntityError {
    errors: HashMap<String, Vec<String>>,
}

impl fmt::Display for UnprocessableEntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let j = serde_json::to_string(&self).unwrap();
        write!(f, "{j}")
    }
}

impl Api {
    /// Constructs an `Api` with the given `api_key` and optional `endpoint`.
    pub fn new(api_options: ApiOptions) -> Self {
        let cert_admin_api = reqwest::Certificate::from_pem(CERT_ADMIN_API).unwrap();
        let cert_peridio = reqwest::Certificate::from_pem(CERT_PERIDIO).unwrap();
        let cert_nerveshub = reqwest::Certificate::from_pem(CERT_NERVESHUB).unwrap();

        let client_builder = ClientBuilder::new()
            .add_root_certificate(cert_admin_api)
            .add_root_certificate(cert_peridio)
            .add_root_certificate(cert_nerveshub)
            .use_rustls_tls();

        let client_builder = if let Some(ca_bundle_path) = api_options.ca_bundle_path {
            if ca_bundle_path.exists() && ca_bundle_path.is_file() {
                let mut buf = Vec::new();
                File::open(ca_bundle_path)
                    .expect("Error opening the ca bundle file")
                    .read_to_end(&mut buf)
                    .expect("Error reading the ca bundle file contents");
                let car_bundle_cert = reqwest::Certificate::from_pem(&buf).unwrap();
                client_builder.add_root_certificate(car_bundle_cert)
            } else {
                panic!("The ca bundle path you provided is invalid");
            }
        } else {
            client_builder
        };

        let client = client_builder.build().unwrap();

        Self {
            api_key: api_options.api_key,
            endpoint: api_options
                .endpoint
                .unwrap_or_else(|| LATEST_ENDPOINT.into()),
            http: client,
        }
    }

    async fn execute<P, T>(
        &self,
        method: Method,
        path: P,
        body: Option<BodyType>,
    ) -> Result<Option<T>, Error>
    where
        P: AsRef<str> + Display,
        T: DeserializeOwned,
    {
        self.execute_with_params_and_headers(method, path, body, vec![], vec![])
            .await
    }

    async fn execute_with_params<P, T>(
        &self,
        method: Method,
        path: P,
        body: Option<BodyType>,
        params: Vec<(String, String)>,
    ) -> Result<Option<T>, Error>
    where
        P: AsRef<str> + Display,
        T: DeserializeOwned,
    {
        self.execute_with_params_and_headers(method, path, body, params, vec![])
            .await
    }

    async fn execute_with_headers<P, T>(
        &self,
        method: Method,
        path: P,
        body: Option<BodyType>,
        headers: Vec<(String, String)>,
    ) -> Result<Option<T>, Error>
    where
        P: AsRef<str> + Display,
        T: DeserializeOwned,
    {
        self.execute_with_params_and_headers(method, path, body, vec![], headers)
            .await
    }

    async fn execute_with_params_and_headers<P, T>(
        &self,
        method: Method,
        path: P,
        body: Option<BodyType>,
        params: Vec<(String, String)>,
        headers: Vec<(String, String)>,
    ) -> Result<Option<T>, Error>
    where
        P: AsRef<str> + Display,
        T: DeserializeOwned,
    {
        let endpoint = format!("{}{}", self.endpoint, path);
        let mut hmap = HeaderMap::new();
        let iter = headers.iter();
        for (k, v) in iter {
            hmap.insert(
                HeaderName::from_bytes(k.as_bytes()).unwrap(),
                HeaderValue::from_bytes(v.as_bytes()).unwrap(),
            );
        }
        let req_builder = self
            .http
            .request(method.clone(), endpoint)
            .query(&params)
            .header("Authorization", format!("Token {}", &self.api_key))
            .headers(hmap);

        let req = match body {
            Some(BodyType::Body((content_type, body))) => req_builder
                .header(header::CONTENT_TYPE, content_type)
                .body(body)
                .build(),
            Some(BodyType::Multipart(multipart)) => req_builder.multipart(multipart).build(),
            None => req_builder.build(),
        };

        let res = self
            .http
            .execute(req.context(BadRequestParams)?)
            .await
            .context(RequestFailed)?;

        match res.status().as_u16() {
            200..=201 => Ok(Some(res.json().await.context(BadResponse)?)),
            204 => Ok(None),
            _ => {
                let response = res.text().await.context(BadResponse)?;
                Err(Error::Unknown { error: response })
            }
        }
    }

    pub fn artifacts(&self) -> ArtifactsApi {
        ArtifactsApi(self)
    }

    pub fn artifact_versions(&self) -> ArtifactVersionsApi {
        ArtifactVersionsApi(self)
    }

    pub fn bundles(&self) -> BundlesApi {
        BundlesApi(self)
    }

    pub fn binaries(&self) -> BinariesApi {
        BinariesApi(self)
    }

    pub fn binary_parts(&self) -> BinaryPartsApi {
        BinaryPartsApi(self)
    }

    pub fn binary_signatures(&self) -> BinarySignaturesApi {
        BinarySignaturesApi(self)
    }

    pub fn ca_certificates(&self) -> CaCertificatesApi {
        CaCertificatesApi(self)
    }

    pub fn cohorts(&self) -> CohortsApi {
        CohortsApi(self)
    }

    pub fn devices(&self) -> DevicesApi {
        DevicesApi(self)
    }

    pub fn device_certificates(&self) -> DeviceCertificatesApi {
        DeviceCertificatesApi(self)
    }

    pub fn deployments(&self) -> DeploymentsApi {
        DeploymentsApi(self)
    }

    pub fn firmwares(&self) -> FirmwaresApi {
        FirmwaresApi(self)
    }

    pub fn organization_users(&self) -> OrganizationUsersApi {
        OrganizationUsersApi(self)
    }

    pub fn products(&self) -> ProductApi {
        ProductApi(self)
    }

    pub fn products_v2(&self) -> ProductsV2Api {
        ProductsV2Api(self)
    }

    pub fn product_users(&self) -> ProductUsersApi {
        ProductUsersApi(self)
    }

    pub fn releases(&self) -> ReleasesApi {
        ReleasesApi(self)
    }

    pub fn signing_keys(&self) -> SigningKeysApi {
        SigningKeysApi(self)
    }

    pub fn users(&self) -> UsersApi {
        UsersApi(self)
    }
}
