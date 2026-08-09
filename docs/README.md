# Contract documentation

This directory holds the design notes and specification for the `procura-escrow`
contract. The authoritative technical specification for the whole platform lives with
the Procura design docs and covers:

- **Storage model** — instance vs persistent storage, `DataKey` layout.
- **Data types** — `Config`, `Engagement`, `Milestone`, and the state enums.
- **Entry points** — `initialize_engagement`, `fund_engagement`, `submit_milestone`,
  `approve_milestone`, `reject_milestone`, `cancel_engagement`, plus admin/read ops.
- **State machine** — engagement (`Draft → Funded → Completed | Cancelled`) and
  milestone (`Pending → Submitted → Approved → Paid`, or `Rejected`) transitions.
- **Authorization model** — the two-gate design (`require_auth` + identity match).
- **Events** — the append-only log consumed by the backend indexer.
- **Error model** — the stable `#[contracterror]` codes.
- **Invariants** — custody conservation, no overpayment, authorization soundness.

Implementation of the contract will land under `../contracts/escrow`.
