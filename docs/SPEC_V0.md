# KEXA-Lite Protocol Spec v0

## Overview
KEXA-Lite v0 is a minimal UTXO-based chain intended for local devnet/testnet usage. It intentionally omits privacy, smart contracts, tokens, staking, or governance. The design aims for low complexity and a low exploit surface.

## Encoding
All consensus-critical objects use deterministic binary serialization via **borsh** with a **version prefix**.

- Version prefix: 1 byte, currently `0x00`.
- Object bytes: `version || borsh(object)`.

Golden tests in `kexa-proto` verify stable serialization output.

## Hashing
- `sha256(data)` is used for transaction IDs.
- Block header hash is **double-SHA256** of serialized header bytes.

## Address Format
- Format: **Bech32**.
- HRP (prefix): `kexa`.
- Payload: `sha256(pubkey)` (32 bytes).
- Checksum: Bech32 checksum.

## Transaction Format
```
Transaction {
  version: u8,
  inputs: Vec<TxIn>,
  outputs: Vec<TxOut>,
}

TxIn {
  outpoint: OutPoint,
  signature: [u8; 64],
  pubkey: [u8; 32],
}

OutPoint {
  txid: [u8; 32],
  index: u32,
}

TxOut {
  amount: u64,
  address: [u8; 32],
}
```

### Transaction ID
`txid = sha256(serialize(tx))`

### Signing
The signing message is the hash of the transaction with **all input signatures zeroed**:
```
message = sha256(serialize(tx with signature=0 for all inputs))
```
Each input uses Ed25519 over this message.
The input `pubkey` must hash to the **same address payload** as the referenced UTXO output.

### Fee
`fee = sum(inputs) - sum(outputs)`

## Block Format
```
Block {
  header: BlockHeader,
  txs: Vec<Transaction>,
}

BlockHeader {
  version: u8,
  prev_hash: [u8; 32],
  merkle_root: [u8; 32],
  timestamp: u64,
  bits: u32,
  nonce: u64,
  height: u64,
}
```

### Block ID
`blockid = double_sha256(serialize(header))`

### Merkle Root
Merkle root is computed over **transaction IDs**:
- Pair-wise hash `sha256(left || right)`.
- If odd, duplicate the last hash.

## Coinbase
- Exactly one coinbase per block.
- Must be the first transaction in the block.
- Coinbase has **no inputs**.
- Coinbase maturity: **0** (devnet convenience).
- Coinbase output sum must be **<= subsidy + total_fees** for the block.

## Block Subsidy
- Constant subsidy: **50 KEXA** per block (devnet/testnet only).

## Difficulty
- Fixed difficulty for devnet: `bits = 16`.
- Interpretation: block hash must have **16 leading zero bits**.

## Mempool Rules
- Reject invalid signatures.
- Reject spends of non-existent UTXOs.
- Reject outputs exceeding inputs.
- Reject double-spends within mempool.
- Within a block, an outpoint may be spent at most once.

## Networking Messages (v0)
Minimal P2P messages (borsh + length prefix):
- `Version { height, tip }`
- `GetBlocks { start_height }`
- `Block { block }`
- `GetBlock { hash }`
- `GetTip`
- `Tip { height, tip }`

Message size limit: **2 MiB**.

## RPC Endpoints
- `GET /health` — liveness
- `GET /ready` — readiness
- `GET /tip` — `{height, hash}`
- `GET /block/:hash` — block payload
- `GET /balance/:address` — `u64` balance
- `GET /utxos/:address` — list of UTXOs
- `POST /submit_tx` — submit transaction
- `POST /mine_blocks` — mine N blocks
- `GET /peers` — peer list
