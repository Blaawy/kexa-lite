# KEXA / KEXA-Lite v0 — Testnet

## Seed
- P2P: 193.123.75.158:9030

## Default ports
- P2P (public): 9030
- RPC (local): 8030

## Chain identity (genesis)
Genesis block hash (height 0):
- 1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159

Expected genesis properties:
- timestamp: 0
- bits: 16
- coinbase subsidy: 50 to zero-address

## Join testnet (example)
Run a node:
```bash
kexa-node \
  --rpc-addr 127.0.0.1:8030 \
  --p2p-addr 0.0.0.0:9030 \
  --data-dir ./kexa-testnet-data \
  --peers "193.123.75.158:9030"
```

## Verify you’re on the right chain
```bash
curl -s http://127.0.0.1:8030/block/1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159
```

## Local status helper
```bash
bash scripts/testnet_status.sh
```
## Mini-Explorer (CLI)

After your node is running (RPC is local), you can browse the chain without guessing hashes:

```bash
# build kexa-cli (no Rust installed)
docker run --rm -v "$PWD":/app -w /app rust:1.85 bash -c "cargo build -p kexa-cli --release"

# browse via your local RPC (example: 127.0.0.1:8030)
./target/release/kexa-cli --rpc http://127.0.0.1:8030 health
./target/release/kexa-cli --rpc http://127.0.0.1:8030 tip
./target/release/kexa-cli --rpc http://127.0.0.1:8030 blocks --last 20
./target/release/kexa-cli --rpc http://127.0.0.1:8030 block --height 0
```

RPC endpoints used:
- `GET /blocks?limit=N`
- `GET /block/:hash`


## Windows quick verify

Seed (P2P): `193.123.75.158:9030`  
Local RPC default: `http://127.0.0.1:8030`

Run:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\local_status.ps1
powershell -ExecutionPolicy Bypass -File .\scripts\join_verify_testnet.ps1
```
