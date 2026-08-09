# Security Policy

Procura is a financial application. The `procura-escrow` contract in this repository
custodies escrowed funds, so we take security reports seriously and appreciate
responsible disclosure.

## Supported versions

Procura is pre-release (testnet). Security fixes are applied to the `main` branch.
There is no supported production/mainnet deployment yet.

## Reporting a vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Instead, report privately through one of:

- **GitHub Security Advisories** — use the
  ["Report a vulnerability"](https://github.com/ProcuraLabs/procura-contracts/security/advisories/new)
  button on this repository's *Security* tab (preferred).
- **Email** — `security@procuralabs.example` (replace with the project's real
  security contact before launch).

Please include:

- A description of the issue and its impact (e.g. fund loss, unauthorized transfer,
  denial of service, state corruption).
- Steps to reproduce or a proof-of-concept, if possible.
- The affected contract/entry point, commit hash, and network.

## What to expect

- Acknowledgement of your report within **3 business days**.
- An initial assessment and severity classification.
- Coordinated disclosure: we will agree on a timeline for a fix and public
  disclosure. We are happy to credit reporters who wish to be named.

## Scope

In scope:

- The `procura-escrow` contract and any supporting contracts in this repository.
- Authorization bypasses, fund-loss or fund-lock bugs, integer/arithmetic issues,
  state-machine violations, and event/indexing integrity issues.

Out of scope:

- Vulnerabilities in third-party dependencies (report those upstream, but do let us
  know if we are affected).
- Issues requiring physical access to a user's device or their wallet secret keys.
- Testnet-only griefing that has no mainnet fund-safety impact (still welcome, but
  triaged at lower priority).

## Security model

Procura is deliberately **non-custodial**: the platform operator and backend hold no
key that can move escrowed funds. Fund custody and release are governed entirely by
this contract. A summary of the trust and threat model is maintained in the Procura
technical specification. Before any mainnet deployment, this contract is subject to
an external audit.
