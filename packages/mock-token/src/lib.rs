#![no_std]
//! # procura-mock-token
//!
//! A test-only SEP-41 token used to exercise Procura's escrow payment path in
//! local tests. This crate is currently a scaffold: it wires the token into the
//! workspace build and test harness but implements **no** balance, allowance, or
//! transfer logic yet. The SEP-41 interface lands alongside the escrow contract.
//!
//! It is never deployed to a public network; it exists only for `cargo test`.

use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct MockToken;

#[contractimpl]
impl MockToken {
    /// Scaffold ABI version. Replaced by the SEP-41 token interface (mint /
    /// balance / transfer) once the escrow tests need it.
    pub fn version() -> u32 {
        0
    }
}

#[cfg(test)]
mod test;
