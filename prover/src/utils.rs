use axum::http::StatusCode;
use sp1_sdk::SP1ProvingKey;
use zkemail_helpers::{RegexConfig, RegexPattern};

use crate::types::DecomposedRegex;

pub struct DecomposedRegexVec(pub Vec<DecomposedRegex>);

impl TryFrom<DecomposedRegexVec> for RegexConfig {
    type Error = &'static str;

    fn try_from(DecomposedRegexVec(regexes): DecomposedRegexVec) -> Result<Self, Self::Error> {
        let mut header_parts = Vec::new();
        let mut body_parts = Vec::new();

        for regex in regexes {
            let mut pattern = String::new();
            let mut capture_indices = Vec::new();
            for (idx, part) in regex.parts.iter().enumerate() {
                if part.is_public {
                    pattern.push_str(&format!("({})", part.regex_def));
                    capture_indices.push(idx + 1);
                } else {
                    pattern.push_str(&part.regex_def);
                }
            }
            let regex_pattern = RegexPattern {
                pattern,
                capture_indices: Some(capture_indices),
            };
            match regex.location.as_str() {
                "header" => header_parts.push(regex_pattern),
                "body" => body_parts.push(regex_pattern),
                _ => return Err("Invalid regex location"),
            }
        }

        Ok(RegexConfig {
            header_parts: Some(header_parts),
            body_parts: Some(body_parts),
        })
    }
}

pub fn get_proving_key() -> Result<SP1ProvingKey, StatusCode> {
    let key_path = std::env::var("PROVING_KEY_PATH")
        .unwrap_or_else(|_| "/app/data/email_with_regex.bin".to_string());

    let key_bytes = std::fs::read(key_path).map_err(|err| {
        tracing::error!("Error reading proving key: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    bincode::deserialize(&key_bytes).map_err(|err| {
        tracing::error!("Error deserializing proving key: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
