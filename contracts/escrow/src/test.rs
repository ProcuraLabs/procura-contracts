#![cfg(test)]

use soroban_sdk::Env;

use crate::{EscrowContract, EscrowContractClient};

/// Smoke test: the scaffold contract registers, invokes, and returns its version.
/// This exercises the full Soroban test harness without asserting any business
/// behavior, which does not exist yet.
#[test]
fn version_returns_scaffold_value() {
    let env = Env::default();
    let contract_id = env.register(EscrowContract, ());
    let client = EscrowContractClient::new(&env, &contract_id);

    assert_eq!(client.version(), 0);
}
