# KEXA-Lite v0

KEXA-Lite is a minimal UTXO-based devnet/testnet chain built for low complexity and a small attack surface. v0 intentionally excludes privacy, smart contracts, tokens, staking, and governance.

## Quickstart (One-Click)
```bash
./kexa dev
```

This command runs:
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `docker compose up --build -d`
- spins up a 2-node devnet
- creates wallets Alice/Bob
- mines blocks, sends a transaction, mines confirmation
- prints balances before/after

## Requirements
- Rust stable
- Docker + Docker Compose

## Scripts
- `./kexa dev`
- `./kexa down`
- `./kexa clean`

## Docs
- `docs/SPEC_V0.md`
- `docs/ARCHITECTURE.md`
- `docs/THREAT_MODEL.md`
- `docs/SECURITY.md`
- `docs/CONTRIBUTING.md`
- `docs/ROADMAP.md`
- `docs/LAUNCH_TESTNET.md`

## Public Seed Node (Testnet / Seed)
- Seed address: `193.123.75.158:9030`
- Notes: P2P is public on 9030. RPC is intentionally not exposed publicly.
