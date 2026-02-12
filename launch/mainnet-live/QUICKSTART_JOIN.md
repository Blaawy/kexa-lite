Checkpoint: Feb 12, 2026 (Asia/Dubai) — CANONICAL HANDOFF v12

# QUICKSTART: Join Mainnet

```bash
# 1) verify artifact integrity
sha256sum -c SHA256SUMS

# 2) verify deterministic genesis identity
kexa-node --network mainnet --genesis /etc/kexa/genesis-mainnet.json --print-genesis

# 3) run node with canonical mainnet flags
kexa-node \
  --network mainnet \
  --genesis /etc/kexa/genesis-mainnet.json \
  --rpc-addr 127.0.0.1:18040 \
  --p2p-addr 0.0.0.0:9040 \
  --data-dir /var/lib/kexa/mainnet \
  --peers "193.123.75.158:9040,141.145.159.171:9040"
```

Expected seeds:
- `193.123.75.158:9040`
- `141.145.159.171:9040`

Warning: RPC is localhost-only; do not expose `18040` publicly.
