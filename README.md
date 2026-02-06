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

## Join Testnet

### Quick verify (10 seconds)

After your node is running (RPC is local):

```bash
# build kexa-cli (no Rust installed)
docker run --rm -v "$PWD":/app -w /app rust:1.85 bash -c "cargo build -p kexa-cli --release"

# verify instantly
./target/release/kexa-cli --rpc http://127.0.0.1:8030 health
./target/release/kexa-cli --rpc http://127.0.0.1:8030 tip
./target/release/kexa-cli --rpc http://127.0.0.1:8030 blocks --last 20
```


### Public seed
- Seed: `193.123.75.158:9030`
- Notes: P2P is public on 9030. RPC is intentionally not exposed publicly.

### Run a node (RPC private)
`kexa-node` flags (exact):
- `--rpc-addr` (default: `127.0.0.1:8030`)
- `--p2p-addr` (default: `0.0.0.0:9030`)
- `--data-dir` (default: `./data`)
- `--peers` = comma-separated list of `ip:port` (example: `"ip1:port,ip2:port"`)

Example (connect to the public seed):
```bash
./target/release/kexa-node --rpc-addr 127.0.0.1:8030 --p2p-addr 0.0.0.0:9030 --data-dir ./data --peers "193.123.75.158:9030"
```

Verify locally:
```bash
curl -s http://127.0.0.1:8030/health
curl -s http://127.0.0.1:8030/peers
curl -s http://127.0.0.1:8030/tip
```

## Mini-Explorer (CLI)

The RPC now supports browsing recent blocks:

- `GET /blocks?limit=N` → last N blocks from tip (summary: height/hash/tx_count/timestamp)
- `GET /block/:hash` → full block by hash


### Build `kexa-cli` (no Rust installed)

```bash
docker run --rm -v "$PWD":/app -w /app rust:1.85 bash -c "cargo build -p kexa-cli --release"
```

Binary will be at:
- `./target/release/kexa-cli`

### Verify testnet activity in seconds

```bash
./target/release/kexa-cli --rpc http://127.0.0.1:8030 health
./target/release/kexa-cli --rpc http://127.0.0.1:8030 tip
./target/release/kexa-cli --rpc http://127.0.0.1:8030 blocks --last 20
./target/release/kexa-cli --rpc http://127.0.0.1:8030 block --height 0
```

