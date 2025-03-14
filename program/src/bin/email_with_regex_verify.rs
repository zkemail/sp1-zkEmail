#![no_main]
sp1_zkvm::entrypoint!(main);

use zkemail_core::{
    verify_email_with_regex, EmailWithRegex, EmailWithRegexVerifierOutput, VerificationOutput,
};

fn main() {
    let input = sp1_zkvm::io::read::<EmailWithRegex>();
    let output: EmailWithRegexVerifierOutput = verify_email_with_regex(&input);
    let output = VerificationOutput::from_parts(output.email, Some(output.regex_matches));
    let output = output.abi_encode();
    sp1_zkvm::io::commit_slice(&output);
}
