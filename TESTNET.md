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
