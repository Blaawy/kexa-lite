# Launching a Small Testnet

## Prerequisites
- Rust stable
- Docker + Docker Compose

## Steps
1. Build binaries:
   ```bash
   cargo build --release
   ```
2. Start a seed node:
   ```bash
   ./target/release/kexa-node --rpc-addr 0.0.0.0:8030 --p2p-addr 0.0.0.0:9030 --data-dir ./node1
   ```
3. Start peer nodes pointing at the seed:
   ```bash
   ./target/release/kexa-node --rpc-addr 0.0.0.0:8031 --p2p-addr 0.0.0.0:9031 --data-dir ./node2 --peers 127.0.0.1:9030
   ```
4. Mine blocks to fund addresses:
   ```bash
   curl -X POST http://127.0.0.1:8030/mine_blocks \
     -H 'Content-Type: application/json' \
     -d '{"count": 1, "miner_address": "kexa1..."}'
   ```

## Safety
- Use firewalls if exposing RPC beyond localhost.
- Keep wallets offline where possible.
- Back up data directories before upgrades.
