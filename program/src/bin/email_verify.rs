#![no_main]
sp1_zkvm::entrypoint!(main);

use zkemail_core::{verify_email, Email, EmailVerifierOutput};

fn main() {
    let input = sp1_zkvm::io::read::<Email>();
    let output: EmailVerifierOutput = verify_email(&input);
    sp1_zkvm::io::commit::<EmailVerifierOutput>(&output);
}
