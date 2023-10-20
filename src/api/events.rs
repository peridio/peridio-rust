use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub data: EventType,
    pub inserted_at: String,
    pub prn: String,
    pub version: u16,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum EventType {
    AuditLog(AuditLogEvent),
    Device(DeviceEvent),
    Webhook(WebhookEvent),
}

// sub-event structs

#[derive(Debug, Deserialize, Serialize)]
pub struct AuditLogEvent {
    pub data: AuditLogEventType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceEvent {
    pub data: DeviceEventType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookEvent {
    pub data: WebhookEventType,
}

// sub-event types

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "data")]
pub enum AuditLogEventType {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "data")]
pub enum DeviceEventType {
    CheckedForRelease(CheckedForReleaseEvent),
    ClaimedRelease(ClaimedReleaseEvent),
    ReleaseChanged(ReleaseChangedEvent),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "data")]
pub enum WebhookEventType {
    TestFire(TestFireEvent),
    RequestFailed(RequestFailedEvent),
}

// audit_log

// audit_log.api_request

#[derive(Debug, Deserialize, Serialize)]
pub struct APIRequestEvent {}

// device

// device.checked_for_release

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckedForReleaseEvent {}

// device.claimed_release

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimedReleaseEvent {}

// device.release_changed

#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseChangedEvent {}

// webhook

// webhook.test_fire

#[derive(Debug, Deserialize, Serialize)]
pub struct TestFireEvent {
    pub webhook_prn: String,
}

// webhook.request_failed

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestFailedEvent {
    pub data: RequestFailedType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "data")]
pub enum RequestFailedType {
    HostResolutionFailed(HostResolutionFailedEvent),
    ResponseStatus(ResponseStatusEvent),
    ResponseTimeout(ResponseTimeoutEvent),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HostResolutionFailedEvent {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseStatusEvent {
    pub status: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseTimeoutEvent {}
