use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecomposedRegexPart {
    pub is_public: bool,
    pub regex_def: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecomposedRegex {
    pub parts: Vec<DecomposedRegexPart>,
    pub name: String,
    pub max_length: usize,
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProverInput {
    pub from_domain: String,
    pub raw_email: String,
    pub regex_info: Vec<DecomposedRegex>,
}
