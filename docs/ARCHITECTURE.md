# KEXA-Lite Architecture

## Workspace Overview
- `kexa-proto`: consensus-critical types, hashing, and serialization.
- `kexa-consensus`: merkle, PoW rules, constants.
- `kexa-storage`: sled-backed persistent storage.
- `kexa-p2p`: message definitions and framing.
- `kexa-node`: daemon with RPC, mempool, mining, p2p.
- `kexa-wallet`: CLI wallet and signing.
- `kexa-testkit`: integration tests and harness.

## Node Pipeline
1. **RPC** accepts transactions and validates them against the UTXO set and mempool.
2. **Mempool** holds valid transactions until mined.
3. **Mining** builds blocks with a coinbase and pending txs, then searches for a nonce to satisfy PoW.
4. **Storage** persists blocks, headers, height mapping, and UTXO set.
5. **P2P** exchanges tips and missing blocks using simple request/response messages.

## Data Model
- UTXO set keyed by `(txid, index)`.
- Blocks indexed by `hash` and by `height -> hash`.
- Tip stored in `meta`.

## Devnet Flow
- Genesis block is created at first startup.
- Blocks can be mined via RPC or `--mine` flag.
- Nodes sync by requesting blocks from a peer’s tip height.
