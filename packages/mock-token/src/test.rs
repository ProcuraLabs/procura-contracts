#![cfg(test)]

use soroban_sdk::Env;

use crate::{MockToken, MockTokenClient};

/// Smoke test: the scaffold token registers and invokes cleanly. Real SEP-41
/// behavior (mint / balance / transfer) is added when the escrow tests need it.
#[test]
fn version_returns_scaffold_value() {
    let env = Env::default();
    let contract_id = env.register(MockToken, ());
    let client = MockTokenClient::new(&env, &contract_id);

    assert_eq!(client.version(), 0);
}
