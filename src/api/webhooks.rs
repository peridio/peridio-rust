use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::events::Event;
use crate::json_body;
use crate::Api;

use super::Error;
use snafu::ResultExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Webhook {
    pub description: Option<String>,
    pub enabled_events: Vec<String>,
    pub inserted_at: String,
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub secret: Option<String>,
    pub state: Option<String>,
    pub updated_at: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateWebhookParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enabled_events: Option<Vec<String>>,
    pub organization_prn: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWebhookResponse {
    pub webhook: Webhook,
}

#[derive(Debug, Serialize)]
pub struct GetWebhookParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetWebhookResponse {
    pub webhook: Webhook,
}

#[derive(Debug, Serialize)]
pub struct ListWebhooksParams {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: String,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListWebhooksResponse {
    pub webhooks: Vec<Webhook>,
    pub next_page: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateWebhookParams {
    pub prn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enabled_events: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWebhookResponse {
    pub webhook: Webhook,
}

#[derive(Debug, Serialize)]
pub struct DeleteWebhookParams {
    pub webhook_prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteWebhookResponse {}

#[derive(Debug, Serialize)]
pub struct RollSecretWebhookParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RollSecretWebhookResponse {
    pub webhook: Webhook,
}

#[derive(Debug, Serialize)]
pub struct TestFireWebhookParams {
    pub prn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestFireWebhookResponse {
    pub event: Event,
}

pub struct WebhooksApi<'a>(pub &'a Api);

impl<'a> WebhooksApi<'a> {
    pub async fn create(
        &'a self,
        params: CreateWebhookParams,
    ) -> Result<Option<CreateWebhookResponse>, Error> {
        self.0
            .execute(Method::POST, "/webhooks", Some(json_body!(&params)))
            .await
    }

    pub async fn get(
        &'a self,
        params: GetWebhookParams,
    ) -> Result<Option<GetWebhookResponse>, Error> {
        let webhook_prn: String = params.prn;
        self.0
            .execute(Method::GET, format!("/webhooks/{webhook_prn}"), None)
            .await
    }

    pub async fn list(
        &'a self,
        params: ListWebhooksParams,
    ) -> Result<Option<ListWebhooksResponse>, Error> {
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
            .execute_with_params(Method::GET, "/webhooks".to_string(), None, query_params)
            .await
    }

    pub async fn update(
        &'a self,
        params: UpdateWebhookParams,
    ) -> Result<Option<UpdateWebhookResponse>, Error> {
        let webhook_prn: &String = &params.prn;

        self.0
            .execute(
                Method::PATCH,
                format!("/webhooks/{webhook_prn}"),
                Some(json_body!(&params)),
            )
            .await
    }

    pub async fn delete(
        &'a self,
        params: DeleteWebhookParams,
    ) -> Result<Option<DeleteWebhookResponse>, Error> {
        let webhook_prn: String = params.webhook_prn;
        self.0
            .execute(Method::DELETE, format!("/webhooks/{webhook_prn}"), None)
            .await
    }

    pub async fn roll_secret(
        &'a self,
        params: RollSecretWebhookParams,
    ) -> Result<Option<RollSecretWebhookResponse>, Error> {
        let webhook_prn: String = params.prn;
        self.0
            .execute(
                Method::POST,
                format!("/webhooks/{webhook_prn}/roll_secret"),
                None,
            )
            .await
    }

    pub async fn test_fire(
        &'a self,
        params: TestFireWebhookParams,
    ) -> Result<Option<TestFireWebhookResponse>, Error> {
        let webhook_prn: String = params.prn;
        self.0
            .execute(
                Method::POST,
                format!("/webhooks/{webhook_prn}/test_fire"),
                None,
            )
            .await
    }
}
