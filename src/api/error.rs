use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApiError {
    // Standard API error format with detail
    Standard {
        errors: StandardErrors,
    },

    // Validation errors format (like 422 responses)
    Validation {
        errors: HashMap<String, Vec<String>>,
    },

    // Simple status format (like 403 responses)
    Status {
        status: String,
    },

    // Generic JSON value for any other structure
    Generic(serde_json::Value),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StandardErrors {
    pub detail: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Standard { errors } => write!(f, "{}", errors.detail),
            ApiError::Validation { errors } => {
                let mut messages = Vec::new();
                for (field, field_errors) in errors {
                    for error in field_errors {
                        messages.push(format!("{}: {}", field, error));
                    }
                }
                write!(f, "Validation errors: {}", messages.join(", "))
            }
            ApiError::Status { status } => write!(f, "Status: {}", status),
            ApiError::Generic(value) => {
                write!(f, "{}", serde_json::to_string(value).unwrap_or_default())
            }
        }
    }
}
