# MAINNET Gate M3 — Deterministic Genesis Candidate

## Purpose
Gate M3 defines a deterministic and reproducible MAINNET genesis candidate plus a public genesis package and verification steps.

## Genesis package contents
- `genesis/mainnet.json` canonical genesis spec (network, header fields, coinbase outputs)
- Published reserve bech32 address (inside `coinbase_outputs`)
- Computed genesis hash from `kexa-node --print-genesis`
- Verification procedure below

## Locking procedure
1. Edit and lock `genesis/mainnet.json`.
2. Replace placeholder reserve address with the final published bech32 reserve address.
3. Keep deterministic header values fixed (`version`, `timestamp`, `bits`, `nonce`).
4. Ensure reserve allocation is transparent in genesis output:
   - `FOUNDERS_RESERVE = 270_000 KEXA`

## Reproducibility proof
Run these commands independently on two different machines.

### Machine A
```bash
cargo run -p kexa-node -- --network mainnet --genesis genesis/mainnet.json --print-genesis
```

### Machine B (fresh clone)
```bash
cargo run -p kexa-node -- --network mainnet --genesis genesis/mainnet.json --print-genesis
```

Compare `genesis_hash` output. They must be identical.

## What is locked by this package
- Reserve output amount/address in `coinbase_outputs`
- Header `timestamp`, `bits`, `nonce`, `version`
- Implicit genesis invariants enforced by builder:
  - `prev_hash = 0`
  - `height = 0`
  - coinbase tx `version = 0`, no inputs
  - `merkle_root` derived from transactions

## Final genesis hash


## Locked Mainnet Genesis (Gate M3)

- network: mainnet
- genesis_hash: 692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5
- reserve_amount: 270000
- reserve_address: kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv
