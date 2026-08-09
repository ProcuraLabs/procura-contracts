#![cfg(test)]

use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, IntoVal, TryFromVal, Val};

use crate::{
    Error, EscrowContract, EscrowContractClient, Milestone, MilestoneStatus, ProcurementRequest,
    ProcurementStatus, Proposal, ProposalStatus,
};

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

/// The domain structs are `#[contracttype]`, so they must round-trip losslessly
/// through the Soroban value encoding used for on-chain storage. Exercise the
/// full request → `Val` → request path for each core type.
#[test]
fn domain_types_round_trip_through_val() {
    let env = Env::default();
    let organization = Address::generate(&env);
    let vendor = Address::generate(&env);

    let request = ProcurementRequest {
        id: 1,
        organization,
        status: ProcurementStatus::Open,
        budget: 10_000,
        selected_vendor: Some(vendor.clone()),
        created_at: 42,
    };
    let val: Val = request.clone().into_val(&env);
    assert_eq!(
        ProcurementRequest::try_from_val(&env, &val).unwrap(),
        request
    );

    let proposal = Proposal {
        id: 7,
        procurement_id: 1,
        vendor,
        status: ProposalStatus::Submitted,
        amount: 9_500,
        submitted_at: 43,
    };
    let val: Val = proposal.clone().into_val(&env);
    assert_eq!(Proposal::try_from_val(&env, &val).unwrap(), proposal);

    let milestone = Milestone {
        id: 0,
        procurement_id: 1,
        status: MilestoneStatus::Pending,
        amount: 2_500,
    };
    let val: Val = milestone.clone().into_val(&env);
    assert_eq!(Milestone::try_from_val(&env, &val).unwrap(), milestone);
}

/// The status enums are stored by discriminant, so their numbering is part of the
/// storage format. Pin each value to catch accidental reordering.
#[test]
fn status_discriminants_are_stable() {
    assert_eq!(ProcurementStatus::Open as u32, 0);
    assert_eq!(ProcurementStatus::VendorSelected as u32, 1);
    assert_eq!(ProcurementStatus::Funded as u32, 2);
    assert_eq!(ProcurementStatus::Completed as u32, 3);
    assert_eq!(ProcurementStatus::Cancelled as u32, 4);

    assert_eq!(ProposalStatus::Submitted as u32, 0);
    assert_eq!(ProposalStatus::Accepted as u32, 1);
    assert_eq!(ProposalStatus::Rejected as u32, 2);

    assert_eq!(MilestoneStatus::Pending as u32, 0);
    assert_eq!(MilestoneStatus::Submitted as u32, 1);
    assert_eq!(MilestoneStatus::Approved as u32, 2);
    assert_eq!(MilestoneStatus::Paid as u32, 3);
    assert_eq!(MilestoneStatus::Rejected as u32, 4);
}
