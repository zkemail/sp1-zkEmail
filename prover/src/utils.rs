use axum::http::StatusCode;
use serde_json::json;
use zkemail_helpers::{RegexConfig, RegexPattern};

use crate::types::DecomposedRegex;

#[derive(Clone)]
pub struct DecomposedRegexVec(pub Vec<DecomposedRegex>);

impl TryFrom<DecomposedRegexVec> for RegexConfig {
    type Error = &'static str;

    fn try_from(DecomposedRegexVec(regexes): DecomposedRegexVec) -> Result<Self, Self::Error> {
        let mut header_parts = Vec::new();
        let mut body_parts = Vec::new();

        for regex in regexes {
            let mut pattern = String::new();
            let mut capture_indices = Vec::new();
            let mut capture_index = 1;
            for part in regex.parts.iter() {
                if part.is_public {
                    pattern.push_str(&format!("({})", part.regex_def));
                    capture_indices.push(capture_index);
                    capture_index += 1;
                } else {
                    pattern.push_str(&part.regex_def);
                }
            }

            let capture_indices = if !capture_indices.is_empty() {
                Some(capture_indices)
            } else {
                None
            };

            let regex_pattern = RegexPattern {
                pattern,
                capture_indices,
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

impl DecomposedRegexVec {
    pub fn match_sp1_proof_outpus(
        self,
        mut matches: Vec<String>,
    ) -> Result<serde_json::Value, StatusCode> {
        let mut result = json!({});

        for decomposed_regex in self.0.into_iter() {
            // Count public parts in decompose regex
            let public_parts_count = decomposed_regex
                .parts
                .iter()
                .filter(|part| part.is_public)
                .count();

            // Remove and save public_parts_count elements from matches
            let mut removed_matches = Vec::with_capacity(public_parts_count);
            for _ in 0..public_parts_count {
                if let Some(value) = matches.first() {
                    removed_matches.push(value.clone());
                    matches.remove(0);
                } else {
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }

            // Save matches as key value pair
            result[decomposed_regex.name] = json!(removed_matches);
        }

        Ok(result)
    }
}
