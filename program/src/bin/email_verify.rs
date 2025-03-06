#![no_main]
sp1_zkvm::entrypoint!(main);

use zkemail_core::{verify_email, Email, EmailVerifierOutput, VerificationOutput};

fn main() {
    let input = sp1_zkvm::io::read::<Email>();
    let output: EmailVerifierOutput = verify_email(&input);
    let output = VerificationOutput::from_parts(output, None);
    let output = output.abi_encode();
    sp1_zkvm::io::commit_slice(&output);
}
