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
    sp1_sdk::utils::setup_logger();

    // Initialize prover
    let client = ProverClient::from_env();

    // Prepare stdin
    let mut stdin = SP1Stdin::new();

    let regex_config = RegexConfig::try_from(DecomposedRegexVec(payload.regex_info))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let input = generate_email_with_regex_inputs(
        &payload.from_domain,
        payload.raw_email.as_bytes(),
        &regex_config,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    stdin.write(&input);

    let pk = get_proving_key()?;

    // Generate proof
    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify proof
    client
        .verify(&proof, &pk.vk)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(proof))
}
