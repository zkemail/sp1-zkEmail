use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sp1_sdk::{HashableKey, ProverClient, SP1Stdin};
use tracing::error;
use tracing::info;
use zkemail_core::VerificationOutput;
use zkemail_helpers::{generate_email_with_regex_inputs, AbiDecodable, RegexConfig};

use crate::{types::ProverInput, utils::DecomposedRegexVec};

pub async fn generate_proof(
    Json(payload): Json<ProverInput>,
) -> Result<Json<ProofData>, StatusCode> {
    info!(
        "Generating proof for email from domain: {}",
        payload.from_domain
    );

    // Initialize prover
    let client = ProverClient::from_env();

    // Prepare stdin
    let mut stdin = SP1Stdin::new();

    let decomposed_regex_vec = DecomposedRegexVec(payload.regex_info);

    let regex_config = RegexConfig::try_from(decomposed_regex_vec.clone()).map_err(|err| {
        tracing::error!("Error parsing regex config: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    tracing::info!("Regex config parsed: {:?}", regex_config);

    let input = generate_email_with_regex_inputs(
        &payload.from_domain,
        payload.raw_email.as_bytes(),
        &regex_config,
        payload.external_inputs,
    )
    .await
    .map_err(|err| {
        tracing::error!("Error generating email with regex inputs: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    stdin.write(&input);
    tracing::info!("Input written to stdin");

    let key_path = std::env::var("EMAIL_WITH_REGEX_VERIFY_PATH").map_err(|err| {
        tracing::error!("Error getting EMAIL_WITH_REGEX_VERIFY_PATH: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Read the ELF binary file at runtime instead of compile time
    let email_with_regex_elf = std::fs::read(&key_path).map_err(|err| {
        tracing::error!("Error reading ELF file at {}: {:?}", key_path, err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let (pk, _) = client.setup(&email_with_regex_elf);

    // Generate proof
    let proof = client.prove(&pk, &stdin).groth16().run().map_err(|err| {
        tracing::error!("Error generating proof: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    tracing::info!("Proof generated");

    let output = VerificationOutput::abi_decode(proof.public_values.as_slice()).map_err(|err| {
        error!("Error decoding regex output: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let (email, matches) = match output {
        VerificationOutput::WithRegex { email, matches } => Ok((email, matches)),
        VerificationOutput::EmailOnly(_) => {
            error!("Decoding of sp1 proof output did not return any result");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }?;

    let parsed_public_outputs = decomposed_regex_vec.match_sp1_proof_outpus(matches)?;

    let proof_data = ProofData {
        proof: serde_json::json!({"hex": hex::encode(proof.bytes())}),
        public_outputs: serde_json::json!({
            "outputs": email,
            "outputs_hex": hex::encode(proof.public_values)
        }),
        parsed_public_outputs,
        vkey_hash: pk.vk.bytes32(),
    };

    Ok(Json(proof_data))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProofData {
    pub proof: serde_json::Value,
    pub public_outputs: serde_json::Value,
    pub parsed_public_outputs: serde_json::Value,
    pub vkey_hash: String,
}
