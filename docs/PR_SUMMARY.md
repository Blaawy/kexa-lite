# PR Summary

## What Changed
- Implemented a minimal KEXA-Lite v0 Rust workspace with node, wallet, P2P, consensus, storage, and testkit.
- Added deterministic serialization and protocol specification.
- Added one-click devnet script and Docker support.
- Added CI workflow for fmt, clippy, and tests.
- Hardened validation: UTXO pubkey/address binding, coinbase reward cap, and genesis height-0 rejection tests.
- Added intra-block double-spend rejection and stricter UTXO spending checks.
- Deferred equal-height fork resolution until reorg support is implemented.

## How To Verify
```bash
./kexa dev
```
