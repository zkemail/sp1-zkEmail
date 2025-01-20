#![no_main]
sp1_zkvm::entrypoint!(main);

use zkemail_core::{
    abi_encode_email_with_regex_verifier_output, verify_email_with_regex, EmailWithRegex,
    EmailWithRegexVerifierOutput,
};

fn main() {
    let input = sp1_zkvm::io::read::<EmailWithRegex>();
    let output: EmailWithRegexVerifierOutput = verify_email_with_regex(&input);
    let output = abi_encode_email_with_regex_verifier_output(&output);
    sp1_zkvm::io::commit_slice(&output);
}
