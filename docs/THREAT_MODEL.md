# KEXA-Lite Threat Model (v0)

## Goals
- Reduce attack surface for a minimal devnet/testnet.
- Focus on consensus correctness, parsing safety, and DoS resistance.

## In-Scope Threats
### Parsing & Serialization
- Malformed messages or transactions.
- Oversized P2P payloads.
- Non-deterministic serialization.

**Mitigations**
- Borsh deterministic encoding with explicit version prefix.
- Max P2P message size (2 MiB).
- Golden tests for serialization.

### Consensus Safety
- Invalid signatures or forged inputs.
- Double-spends or negative balances.
- Merkle root mismatch.

**Mitigations**
- Input UTXO existence checks.
- Output sum <= input sum.
- Merkle root verification.
- PoW verification.

### Network DoS
- Large message floods.
- Slow peers.

**Mitigations**
- Size limits on messages.
- Simple request/response model.
- Minimal peer list.

## Out of Scope (v0)
- Eclipse attacks, full peer scoring, and robust bans.
- Privacy/anonymity.
- Smart contracts, tokens, or staking.
- Byzantine fault tolerance beyond PoW.
