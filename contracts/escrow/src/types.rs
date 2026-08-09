//! Core domain types for the `procura-escrow` contract.
//!
//! These are the on-chain data types — the procurement request, its proposals,
//! and its milestones, together with their status enums. They are all
//! `#[contracttype]`, so their field layout and the `u32` discriminants of the
//! status enums are part of the contract's **storage format** and must stay
//! stable across changes.
//!
//! ## Primitive conventions
//!
//! Soroban's contract spec carries concrete primitives, not Rust type aliases, so
//! the fields below use primitives directly under a fixed convention:
//!
//! - **Identifiers** — `u64` for procurement and proposal ids (monotonic,
//!   contract-assigned); `u32` for a milestone id, which is unique only within
//!   its procurement.
//! - **Amounts** — `i128`, matching the SEP-41 balance/transfer type.
//! - **Timestamps** — `u64` Unix seconds, as reported by the ledger.
//! - **Addresses** — `Address`, for buyer organizations and vendors.
//!
//! This change defines the types only. The entry points that create and mutate
//! them — and the state-machine rules governing their transitions — land in later
//! changes.

use soroban_sdk::{contracttype, Address};

/// Lifecycle status of a procurement request.
///
/// Discriminants are stable storage values; transition rules are defined by the
/// state machine in a later change.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcurementStatus {
    /// Accepting proposals; no vendor chosen yet.
    Open = 0,
    /// A proposal has been accepted; awaiting escrow funding.
    VendorSelected = 1,
    /// Escrow is funded and work is underway.
    Funded = 2,
    /// All milestones settled.
    Completed = 3,
    /// Terminated before completion.
    Cancelled = 4,
}

/// Lifecycle status of a proposal submitted against a procurement request.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    /// Submitted by a vendor; under consideration.
    Submitted = 0,
    /// Selected as the winning proposal.
    Accepted = 1,
    /// Not selected.
    Rejected = 2,
}

/// Lifecycle status of a milestone within a funded procurement.
///
/// Mirrors the milestone state machine described in `docs/README.md`
/// (`Pending → Submitted → Approved → Paid`, or `Rejected`).
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneStatus {
    /// Not yet started by the vendor.
    Pending = 0,
    /// Deliverable submitted; awaiting review.
    Submitted = 1,
    /// Accepted by the buyer; payment due.
    Approved = 2,
    /// Payment released from escrow.
    Paid = 3,
    /// Rejected by the buyer.
    Rejected = 4,
}

/// A procurement request opened by a buyer organization.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcurementRequest {
    /// Unique identifier assigned by the contract.
    pub id: u64,
    /// Buyer organization that owns this request.
    pub organization: Address,
    /// Current lifecycle status.
    pub status: ProcurementStatus,
    /// Maximum the organization intends to escrow for this request.
    pub budget: i128,
    /// Vendor whose proposal was accepted, once one has been selected.
    pub selected_vendor: Option<Address>,
    /// Ledger time (Unix seconds) at which the request was created.
    pub created_at: u64,
}

/// A vendor's proposal against a procurement request.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    /// Unique identifier assigned by the contract.
    pub id: u64,
    /// Procurement request this proposal targets.
    pub procurement_id: u64,
    /// Vendor submitting the proposal.
    pub vendor: Address,
    /// Current lifecycle status.
    pub status: ProposalStatus,
    /// Total amount bid for the work.
    pub amount: i128,
    /// Ledger time (Unix seconds) at which the proposal was submitted.
    pub submitted_at: u64,
}

/// A milestone within a funded procurement, carrying a portion of the escrow.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    /// Identifier, unique within the procurement.
    pub id: u32,
    /// Procurement request this milestone belongs to.
    pub procurement_id: u64,
    /// Current lifecycle status.
    pub status: MilestoneStatus,
    /// Amount released from escrow when this milestone is paid.
    pub amount: i128,
}
