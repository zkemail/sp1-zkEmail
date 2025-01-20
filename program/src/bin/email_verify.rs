#![no_main]
sp1_zkvm::entrypoint!(main);

use zkemail_core::{abi_encode_email_verifier_output, verify_email, Email, EmailVerifierOutput};

fn main() {
    let input = sp1_zkvm::io::read::<Email>();
    let output: EmailVerifierOutput = verify_email(&input);
    let output = abi_encode_email_verifier_output(&output);
    sp1_zkvm::io::commit_slice(&output);
}
