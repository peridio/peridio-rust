use serde_json::{self, to_string, Map, Value};
use validator::ValidationError;

pub fn validate_json_byte_length_1_000_000(
    value: &Map<String, Value>,
) -> Result<(), ValidationError> {
    match to_string(value) {
        Ok(value) => {
            if value.len() <= 1_000_000 {
                Ok(())
            } else {
                Err(ValidationError::new("greater than 1MB"))
            }
        }
        Err(_e) => Err(ValidationError::new("invalid json")),
    }
}
