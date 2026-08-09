//! Stable error codes for the `procura-escrow` contract.
//!
//! These `#[contracterror]` variants are part of the contract's public ABI: the
//! integer discriminants are surfaced to callers and consumed by the backend
//! indexer, so the numbering is **stable**. New variants are appended with new
//! numbers; existing numbers are never reordered or reused.
//!
//! This change establishes the error *model* only. The entry points that return
//! these errors — the engagement/milestone state machine, escrow custody, and the
//! SEP-41 payment path — are implemented in later changes.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The caller is not authorized for this operation — it fails the
    /// `require_auth` + identity-match authorization gate.
    NotAuthorized = 1,

    /// A supplied argument is malformed or out of range (e.g. a non-positive
    /// amount or an empty identifier).
    InvalidInput = 2,

    /// No procurement exists for the supplied identifier.
    ProcurementNotFound = 3,

    /// No proposal exists for the supplied identifier.
    ProposalNotFound = 4,

    /// The procurement is not in a state that permits the requested operation.
    InvalidProcurementState = 5,

    /// The proposal is not in a state that permits the requested operation.
    InvalidProposalState = 6,

    /// A vendor has already been selected for this procurement.
    VendorAlreadySelected = 7,

    /// The engagement's escrow has not been funded.
    EscrowNotFunded = 8,

    /// The escrow balance is insufficient for the requested release.
    InsufficientEscrow = 9,

    /// No milestone exists for the supplied identifier.
    MilestoneNotFound = 10,

    /// The milestone is not in a state that permits the requested operation.
    InvalidMilestoneState = 11,

    /// Payment for this milestone has already been released.
    PaymentAlreadyReleased = 12,

    /// The operation has already been applied — an idempotency / replay guard.
    DuplicateOperation = 13,

    /// Cancellation is not permitted in the current state.
    CancellationNotAllowed = 14,
}
