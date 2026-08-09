#![no_std]
//! # procura-escrow
//!
//! On-chain trust anchor for Procura. This crate is the workspace scaffold for the
//! `procura-escrow` contract: it establishes the Soroban SDK, build, and test
//! harness so later changes can add real behavior against a green baseline.
//!
//! The engagement/milestone state machine, escrow custody, authorization model,
//! and SEP-41 payment path are specified in `docs/README.md` and are **not yet
//! implemented**. To keep this scaffold honest, the contract exposes only a single
//! read-only `version` entry point — enough to prove it compiles to WASM and is
//! testable, and nothing that touches funds or state transitions.

use soroban_sdk::{contract, contractimpl};

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
}

#[cfg(test)]
mod test;
