# procura-contracts

> Rust / Soroban smart contracts for **Procura** — a decentralized procurement &
> milestone-payment platform on Stellar.

This repository contains the on-chain trust anchor of Procura: the `procura-escrow`
contract, which custodies escrowed funds and enforces the milestone state machine
that governs when those funds are released.

Procura is built as **three separate repositories**:

| Repo | Stack | Role |
|------|-------|------|
| [`procura-contracts`](https://github.com/ProcuraLabs/procura-contracts) | Rust / Soroban | Escrow + milestone state machine (this repo) |
| [`procura-backend`](https://github.com/ProcuraLabs/procura-backend) | TypeScript / Node.js | Indexer, REST API, transaction builder |
| [`procura-frontend`](https://github.com/ProcuraLabs/procura-frontend) | TypeScript / React / Vite | Organization & vendor dashboards |

> **Status: scaffold.** This repository currently contains project structure,
> tooling, and documentation only. The contract implementation has not landed yet.

---

## What lives on-chain

The contract is the source of truth for **money and the state transitions that move
money**. Everything else (descriptions, files, search) is off-chain in the backend.

- Custody of escrowed funds for each engagement.
- The engagement state machine: `Draft → Funded → Completed | Cancelled`.
- Per-milestone lifecycle: `Pending → Submitted → Approved → Paid` (or `Rejected`).
- Authorization: only the organization can fund/approve/reject/cancel; only the
  selected vendor can submit.
- Token transfer (SEP-41) on milestone approval.
- An append-only event log of every transition (consumed by the backend indexer).

The full contract design — storage model, entry points, state machine,
authorization, events, and error model — is specified in the Procura technical
specification and summarized under [`docs/`](./docs).

## Toolchain

| Component | Version |
|-----------|---------|
| `soroban-sdk` | `27.x` |
| Rust | stable, target `wasm32v1-none` |
| Stellar CLI | latest (`stellar contract ...`) |

## Intended layout

```
procura-contracts/
├── contracts/escrow/        # the procura-escrow contract (to be implemented)
├── packages/mock-token/     # test-only SEP-41 token for local tests
├── deployments/             # per-network { contractId, wasmHash } records
├── scripts/                 # build / deploy / bindings helpers
└── docs/                    # contract specification notes
```

## Getting started (once implemented)

```bash
# Build the optimized WASM
make build

# Run the test suite (unit + authorization + invariant tests)
make test

# Deploy to a network and record the contract id
make deploy NETWORK=testnet
```

## Security

The escrow contract holds funds. Please read [SECURITY.md](./SECURITY.md) before
reporting a vulnerability. Do **not** open public issues for security reports.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

Licensed under the [Apache License 2.0](./LICENSE).
