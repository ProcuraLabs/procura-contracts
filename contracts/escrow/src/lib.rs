#![no_std]
//! # procura-escrow
//!
//! On-chain trust anchor for Procura. This crate is the workspace scaffold for the
//! `procura-escrow` contract: it establishes the Soroban SDK, build, and test
//! harness so later changes can add real behavior against a green baseline.
//!
//! Procurement requests can be created; the escrow custody, milestone state
//! machine, and SEP-41 payment path specified in `docs/README.md` are **not yet
//! implemented**. No entry point here moves funds.

use soroban_sdk::{contract, contractimpl, Address, Env};

mod error;
mod storage;
mod types;

pub use error::Error;
pub use storage::DataKey;
pub use types::{
    Config, Escrow, Milestone, MilestoneStatus, ProcurementRequest, ProcurementStatus, Proposal,
    ProposalStatus,
};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Scaffold ABI version. `0` marks the pre-implementation scaffold; real entry
    /// points (fund / submit / approve / reject / cancel) replace this placeholder
    /// when the escrow state machine lands.
    pub fn version() -> u32 {
        0
    }

    /// Create a new procurement request owned by `organization`.
    ///
    /// The organization must authorize the call. The contract assigns the next
    /// monotonic id, initializes the request in [`ProcurementStatus::Open`] with no
    /// vendor selected, timestamps it from the ledger, and persists it. Returns the
    /// assigned procurement id.
    ///
    /// # Errors
    /// - [`Error::InvalidInput`] if `budget` is not strictly positive.
    /// - [`Error::DuplicateOperation`] if the assigned id is already occupied — a
    ///   storage invariant guard that should never trigger in practice.
    pub fn create_procurement(env: Env, organization: Address, budget: i128) -> Result<u64, Error> {
        // Authorization gate: the caller must prove they are the organization.
        organization.require_auth();

        // Validate inputs: the escrow budget must be a positive amount.
        if budget <= 0 {
            return Err(Error::InvalidInput);
        }

        // Assign the next monotonic id from the instance-scoped counter.
        let last_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::LastProcurementId)
            .unwrap_or(0);
        let id = last_id + 1;

        // Defensive invariant: a freshly assigned id must not already exist.
        let key = DataKey::Procurement(id);
        if env.storage().persistent().has(&key) {
            return Err(Error::DuplicateOperation);
        }

        let request = ProcurementRequest {
            id,
            organization,
            status: ProcurementStatus::Open,
            budget,
            selected_vendor: None,
            created_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&key, &request);
        env.storage()
            .instance()
            .set(&DataKey::LastProcurementId, &id);

        Ok(id)
    }
}

#[cfg(test)]
mod test;
