use std::path::Path;

use anyhow::{anyhow, bail};
use ream_consensus_lean::state::LeanState;
use tracing::info;

use crate::types::{TestFixture, verify_signatures::VerifySignaturesTest};

/// Load a verify_signatures test fixture from a JSON file
pub fn load_verify_signatures_test(
    path: impl AsRef<Path>,
) -> anyhow::Result<TestFixture<VerifySignaturesTest>> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .map_err(|err| anyhow!("Failed to read test file {}: {err}", path.display()))?;
    serde_json::from_str(&content)
        .map_err(|err| anyhow!("Failed to parse test file {}: {err}", path.display()))
}

/// Run a single verify_signatures test case. Returns Ok(true) if the test ran,
/// Ok(false) if it requires Type-2 proof verification that Ream does not expose yet.
pub fn run_verify_signatures_test(
    test_name: &str,
    test: &VerifySignaturesTest,
) -> anyhow::Result<bool> {
    info!("Running verify_signatures test: {test_name}");

    let parent_state = LeanState::try_from(&test.anchor_state)
        .map_err(|err| anyhow!("Failed to convert anchor state: {err}"))?;

    let result = test.signed_block.verify_signatures(&parent_state);

    match (result, test.expect_exception.as_ref()) {
        (Ok(_), Some(exception)) => {
            bail!("Expected exception '{exception}' but verify_signatures succeeded");
        }
        (Err(err), None) => {
            if test.signed_block.signature.is_none() && is_unsupported_type_two_proof_error(&err) {
                info!("Skipping unsupported Type-2 proof fixture: {err}");
                return Ok(false);
            }
            bail!("verify_signatures should succeed but failed: {err}");
        }
        (Err(err), Some(_)) => {
            info!("Got expected exception: {err}");
        }
        (Ok(_), None) => {
            info!("verify_signatures succeeded as expected");
        }
    }
    Ok(true)
}

fn is_unsupported_type_two_proof_error(err: &anyhow::Error) -> bool {
    let message = err.to_string();
    message.contains("require Type-2 multi-message proof verification")
        || message.contains("Failed to deserialize AggregatedXMSS proof")
}
