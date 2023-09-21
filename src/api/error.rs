use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub errors: Errors,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Errors {
    pub detail: String,
}
