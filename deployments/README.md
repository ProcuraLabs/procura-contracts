# Deployments

Each network's deployment is recorded here as a JSON file so the backend and
frontend can reference the correct contract and token addresses.

Expected files (created by `scripts/deploy.sh` once the contract is implemented):

- `local.json`
- `testnet.json`
- `mainnet.json` (future — gated on an external audit)

Each file has the shape:

```json
{
  "network": "testnet",
  "escrowContractId": "C...",
  "escrowWasmHash": "...",
  "tokenContractId": "C...",
  "deployedAt": "2026-01-01T00:00:00Z"
}
```

These files contain **public** on-chain identifiers only. They must never contain
secret keys.
