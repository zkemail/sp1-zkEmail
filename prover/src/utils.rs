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
            // Find if there's any public part and its index
            let public_idx = regex.parts.iter().position(|part| part.is_public);

            let pattern = if let Some(idx) = public_idx {
                // Has a public part - create Capture
                let prefix = regex.parts[..idx]
                    .iter()
                    .map(|p| p.regex_def.clone())
                    .collect::<String>();

                let capture = regex.parts[idx].regex_def.clone();

                let suffix = regex.parts[idx + 1..]
                    .iter()
                    .map(|p| p.regex_def.clone())
                    .collect::<String>();

                RegexPattern::Capture {
                    prefix,
                    capture,
                    suffix,
                }
            } else {
                // No public parts - concatenate all for Match
                let pattern = regex
                    .parts
                    .iter()
                    .map(|p| p.regex_def.clone())
                    .collect::<String>();

                RegexPattern::Match { pattern }
            };

            // Add to appropriate vector based on location
            match regex.location.as_str() {
                "header" => header_parts.push(pattern),
                "body" => body_parts.push(pattern),
                _ => return Err("Invalid regex location"),
            }
        }

        Ok(RegexConfig {
            header_parts,
            body_parts,
        })
    }
}

pub fn get_proving_key() -> Result<SP1ProvingKey, StatusCode> {
    let key_bytes = std::fs::read("/usr/local/bin/proving_key.bin")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    bincode::deserialize(&key_bytes).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
