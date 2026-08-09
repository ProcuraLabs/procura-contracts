# Contributing to procura-contracts

Thanks for your interest in contributing to Procura's smart contracts. This
repository holds the on-chain trust anchor of the platform, so contributions —
especially to contract logic — are held to a high bar for correctness and testing.

## Ground rules

- **The contract governs money.** Any change to `contracts/escrow` that touches
  storage, entry points, authorization, or token transfers must come with tests
  proving the relevant invariants still hold (see below).
- **No secrets in commits.** Never commit secret keys, `.env` files, or deployment
  credentials. See [Secrets](#secrets).
- Discuss non-trivial changes in an issue before opening a large PR.

## Development environment

You will need:

- Rust (stable) with the `wasm32v1-none` target:
  `rustup target add wasm32v1-none`
- The [Stellar CLI](https://developers.stellar.org/docs/tools/cli): `stellar`
- `soroban-sdk` 27.x (pulled via Cargo)

Common commands (see the `Makefile` once implemented):

```bash
make build     # cargo build + stellar contract build (optimized wasm)
make test      # cargo test — unit, authorization, and invariant tests
make fmt       # cargo fmt
make lint      # cargo clippy -D warnings
```

## Testing expectations

Every PR must keep the following green:

- `cargo fmt --check` and `cargo clippy -D warnings`.
- The full `cargo test` suite.

Contract logic changes must additionally preserve these invariants (with tests):

1. **Custody conservation** — the contract's token balance always equals the sum of
   unreleased milestone amounts.
2. **Authorization soundness** — no state transition succeeds without the correct
   party's `require_auth` and an identity match.
3. **No overpayment / Paid is terminal** — a milestone pays at most once.
4. **State monotonicity** — engagement and milestone states only move forward along
   the specified state machine.

Authorization negative tests (wrong party, stranger) are required for any new or
changed entry point.

## Commit & PR conventions

- Use clear, imperative commit messages (Conventional Commits encouraged:
  `feat:`, `fix:`, `test:`, `docs:`, `chore:`).
- Keep PRs focused. Include a short description of the change and its security impact.
- Reference related issues.

## Secrets

- Deployment uses a `DEPLOYER_SECRET` that is **also the contract admin key**. It is
  provided via the environment / CI secret store and must never be committed.
- `.gitignore` excludes `.env*` and key files. If you think you may have committed a
  secret, rotate it immediately and notify the maintainers.

## Code of conduct

Be respectful and constructive. Harassment or abuse will not be tolerated.

## License

By contributing, you agree that your contributions will be licensed under the
[Apache License 2.0](./LICENSE).
