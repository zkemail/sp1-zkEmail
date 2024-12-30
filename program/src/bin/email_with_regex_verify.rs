#![no_main]
sp1_zkvm::entrypoint!(main);

use zkemail_core::{verify_email_with_regex, EmailWithRegex, EmailWithRegexVerifierOutput};

fn main() {
    let input = sp1_zkvm::io::read::<EmailWithRegex>();
    let output: EmailWithRegexVerifierOutput = verify_email_with_regex(&input);
    sp1_zkvm::io::commit::<EmailWithRegexVerifierOutput>(&output);
}
