//! Storage key layout for the `procura-escrow` contract.
//!
//! `DataKey` is the single, exhaustive set of keys the contract reads and writes.
//! Centralizing it here keeps the storage model auditable and collision-free:
//! each `#[contracttype]` enum variant serializes with its own variant symbol, so
//! keys that share a numeric id but differ in kind — e.g. `Procurement(1)` and
//! `Proposal(1)` — never alias the same slot.
//!
//! ## Storage tiers (Soroban conventions)
//!
//! - **Instance** storage — small, singleton, always-loaded data whose lifetime
//!   tracks the contract instance: the [`Config`] and the id counters.
//! - **Persistent** storage — the per-entity records, which grow without bound as
//!   procurements accumulate and each carry their own archival lifetime.
//!
//! This change defines the keys only; the operations that populate these slots
//! land in later changes.
//!
//! [`Config`]: crate::Config

use soroban_sdk::contracttype;

/// Every storage slot the contract uses, keyed by entity and identifier.
///
/// New variants are **appended** as the contract grows; existing variants are
/// never renamed or reordered, since the variant symbol and its arguments form
/// the on-chain key.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Instance · singleton contract [`Config`](crate::Config).
    Config,
    /// Instance · last procurement id assigned; the next id is this `+ 1`.
    LastProcurementId,
    /// Instance · last proposal id assigned; the next id is this `+ 1`.
    LastProposalId,

    /// Persistent · a [`ProcurementRequest`](crate::ProcurementRequest) by its id.
    Procurement(u64),
    /// Persistent · a [`Proposal`](crate::Proposal) by its id.
    Proposal(u64),
    /// Persistent · a [`Milestone`](crate::Milestone) by `(procurement id, milestone id)`.
    Milestone(u64, u32),
    /// Persistent · the [`Escrow`](crate::Escrow) accounting for a procurement,
    /// keyed by procurement id.
    Escrow(u64),
}
