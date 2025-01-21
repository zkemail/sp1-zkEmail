use axum::{http::StatusCode, Json};
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use tracing::info;
use zkemail_helpers::{generate_email_with_regex_inputs, RegexConfig};

use crate::{
    types::ProverInput,
    utils::{get_proving_key, DecomposedRegexVec},
};

pub async fn generate_proof(
    Json(payload): Json<ProverInput>,
) -> Result<Json<SP1ProofWithPublicValues>, StatusCode> {
    info!(
        "Generating proof for email from domain: {}",
        payload.from_domain
    );

    // Initialize prover
    let client = ProverClient::from_env();

    // Prepare stdin
    let mut stdin = SP1Stdin::new();

    let regex_config =
        RegexConfig::try_from(DecomposedRegexVec(payload.regex_info)).map_err(|err| {
            tracing::error!("Error parsing regex config: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    tracing::info!("Regex config parsed: {:?}", regex_config);

    let input = generate_email_with_regex_inputs(
        &payload.from_domain,
        payload.raw_email.as_bytes(),
        &regex_config,
    )
    .await
    .map_err(|err| {
        tracing::error!("Error generating email with regex inputs: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    stdin.write(&input);
    tracing::info!("Input written to stdin");

    let pk = get_proving_key().map_err(|err| {
        tracing::error!("Error getting proving key: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Generate proof
    let proof = client.prove(&pk, &stdin).run().map_err(|err| {
        tracing::error!("Error generating proof: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    tracing::info!("Proof generated");

    // Verify proof
    client.verify(&proof, &pk.vk).map_err(|err| {
        tracing::error!("Error verifying proof: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    tracing::info!("Proof verified");

    Ok(Json(proof))
}
