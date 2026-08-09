#![cfg(test)]

use soroban_sdk::Env;

use crate::{Error, EscrowContract, EscrowContractClient};

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

/// The `#[contracterror]` discriminants are part of the contract's public ABI —
/// the backend indexer maps these numbers to error kinds — so they must stay
/// stable. Pin each code to catch accidental reordering or renumbering.
#[test]
fn error_codes_are_stable() {
    assert_eq!(Error::NotAuthorized as u32, 1);
    assert_eq!(Error::InvalidInput as u32, 2);
    assert_eq!(Error::ProcurementNotFound as u32, 3);
    assert_eq!(Error::ProposalNotFound as u32, 4);
    assert_eq!(Error::InvalidProcurementState as u32, 5);
    assert_eq!(Error::InvalidProposalState as u32, 6);
    assert_eq!(Error::VendorAlreadySelected as u32, 7);
    assert_eq!(Error::EscrowNotFunded as u32, 8);
    assert_eq!(Error::InsufficientEscrow as u32, 9);
    assert_eq!(Error::MilestoneNotFound as u32, 10);
    assert_eq!(Error::InvalidMilestoneState as u32, 11);
    assert_eq!(Error::PaymentAlreadyReleased as u32, 12);
    assert_eq!(Error::DuplicateOperation as u32, 13);
    assert_eq!(Error::CancellationNotAllowed as u32, 14);
}
