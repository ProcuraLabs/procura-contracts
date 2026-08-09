#![cfg(test)]

use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{Address, Env, IntoVal, TryFromVal, Val};

use crate::{
    Config, DataKey, Error, Escrow, EscrowContract, EscrowContractClient, Milestone,
    MilestoneStatus, ProcurementRequest, ProcurementStatus, Proposal, ProposalStatus,
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

/// Store each record under its `DataKey`, then read it back, exercising the real
/// Soroban storage tiers. Also pins the anti-collision property: keys that share a
/// numeric id but differ in kind — `Procurement(1)` vs `Proposal(1)` vs
/// `Escrow(1)` — address independent slots.
#[test]
fn records_round_trip_through_storage() {
    let env = Env::default();
    let contract_id = env.register(EscrowContract, ());

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let organization = Address::generate(&env);
    let vendor = Address::generate(&env);

    let config = Config {
        admin,
        token: token.clone(),
    };
    let request = ProcurementRequest {
        id: 1,
        organization,
        status: ProcurementStatus::Open,
        budget: 10_000,
        selected_vendor: None,
        created_at: 100,
    };
    let proposal = Proposal {
        id: 1,
        procurement_id: 1,
        vendor,
        status: ProposalStatus::Submitted,
        amount: 9_500,
        submitted_at: 101,
    };
    let milestone = Milestone {
        id: 0,
        procurement_id: 1,
        status: MilestoneStatus::Pending,
        amount: 2_500,
    };
    let escrow = Escrow {
        procurement_id: 1,
        funded: 9_500,
        released: 0,
    };

    env.as_contract(&contract_id, || {
        let instance = env.storage().instance();
        let persistent = env.storage().persistent();

        instance.set(&DataKey::Config, &config);
        instance.set(&DataKey::LastProcurementId, &1u64);
        persistent.set(&DataKey::Procurement(1), &request);
        persistent.set(&DataKey::Proposal(1), &proposal);
        persistent.set(&DataKey::Milestone(1, 0), &milestone);
        persistent.set(&DataKey::Escrow(1), &escrow);

        assert_eq!(instance.get::<_, Config>(&DataKey::Config).unwrap(), config);
        assert_eq!(
            instance.get::<_, u64>(&DataKey::LastProcurementId).unwrap(),
            1
        );
        assert_eq!(
            persistent
                .get::<_, ProcurementRequest>(&DataKey::Procurement(1))
                .unwrap(),
            request
        );
        assert_eq!(
            persistent
                .get::<_, Proposal>(&DataKey::Proposal(1))
                .unwrap(),
            proposal
        );
        assert_eq!(
            persistent
                .get::<_, Milestone>(&DataKey::Milestone(1, 0))
                .unwrap(),
            milestone
        );
        assert_eq!(
            persistent.get::<_, Escrow>(&DataKey::Escrow(1)).unwrap(),
            escrow
        );

        // Same id `1`, different key kinds must not alias one another.
        assert!(persistent.has(&DataKey::Procurement(1)));
        assert!(persistent.has(&DataKey::Proposal(1)));
        assert!(persistent.has(&DataKey::Escrow(1)));
        assert!(!persistent.has(&DataKey::Procurement(2)));
        assert!(!persistent.has(&DataKey::Milestone(1, 1)));
    });
}

/// Happy path: an authorized organization creates a procurement. The contract
/// assigns id 1, stamps the ledger time, and persists an `Open` request with no
/// vendor selected.
#[test]
fn create_procurement_succeeds() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_700_000_000);

    let contract_id = env.register(EscrowContract, ());
    let client = EscrowContractClient::new(&env, &contract_id);
    let organization = Address::generate(&env);

    let id = client.create_procurement(&organization, &10_000);
    assert_eq!(id, 1);

    let stored = env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .get::<_, ProcurementRequest>(&DataKey::Procurement(1))
            .unwrap()
    });
    assert_eq!(
        stored,
        ProcurementRequest {
            id: 1,
            organization,
            status: ProcurementStatus::Open,
            budget: 10_000,
            selected_vendor: None,
            created_at: 1_700_000_000,
        }
    );
}

/// The organization must authorize the call; without its signature the
/// `require_auth` gate rejects creation.
#[test]
fn create_procurement_requires_organization_auth() {
    let env = Env::default();
    // Note: no `mock_all_auths()` — the required authorization is absent.
    let contract_id = env.register(EscrowContract, ());
    let client = EscrowContractClient::new(&env, &contract_id);
    let organization = Address::generate(&env);

    let result = client.try_create_procurement(&organization, &10_000);
    assert!(result.is_err());
}

/// A non-positive budget is rejected as invalid input, and nothing is persisted.
#[test]
fn create_procurement_rejects_non_positive_budget() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EscrowContract, ());
    let client = EscrowContractClient::new(&env, &contract_id);
    let organization = Address::generate(&env);

    for budget in [0i128, -1] {
        let result = client.try_create_procurement(&organization, &budget);
        assert_eq!(result, Err(Ok(Error::InvalidInput)));
    }

    // No id was consumed and no record written.
    env.as_contract(&contract_id, || {
        assert!(!env.storage().persistent().has(&DataKey::Procurement(1)));
        assert!(env
            .storage()
            .instance()
            .get::<_, u64>(&DataKey::LastProcurementId)
            .is_none());
    });
}

/// Repeated creation yields distinct, monotonically increasing ids that address
/// independent storage slots — no id collision or record overwrite.
#[test]
fn create_procurement_generates_unique_ids() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EscrowContract, ());
    let client = EscrowContractClient::new(&env, &contract_id);
    let org_a = Address::generate(&env);
    let org_b = Address::generate(&env);

    let first = client.create_procurement(&org_a, &10_000);
    let second = client.create_procurement(&org_b, &25_000);
    assert_eq!(first, 1);
    assert_eq!(second, 2);

    env.as_contract(&contract_id, || {
        let persistent = env.storage().persistent();
        let a = persistent
            .get::<_, ProcurementRequest>(&DataKey::Procurement(1))
            .unwrap();
        let b = persistent
            .get::<_, ProcurementRequest>(&DataKey::Procurement(2))
            .unwrap();
        assert_eq!(a.organization, org_a);
        assert_eq!(b.organization, org_b);
        assert_eq!(b.budget, 25_000);
    });
}
