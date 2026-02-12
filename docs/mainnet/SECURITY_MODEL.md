Checkpoint: Feb 12, 2026 (Asia/Dubai) — CANONICAL HANDOFF v12

# SECURITY MODEL — Mainnet RPC Privacy + Explorer Proxy

## Public vs private surfaces

### Public
- P2P gossip/handshake on TCP `9040`
- Public seeds:
  - `193.123.75.158:9040`
  - `141.145.159.171:9040`
- Public explorer web UI:
  - `http://193.123.75.158/`

### Private
- Node RPC on `127.0.0.1:18040` only
- RPC is not internet-facing

> Hard rule: never expose `18040` publicly.

## Explorer trust boundary (locked)

- Explorer lives in `kexa-explorer/` (Next.js 14 + TypeScript + Tailwind).
- Browser never calls node RPC directly.
- Browser calls explorer routes under `app/api/kexa/*`:
  - `/health`
  - `/tip`
  - `/blocks`
  - `/block/[hash]`
  - `/peers/live`
- Next.js server-side route handlers proxy to localhost RPC.
- Node `/health` returns plain text `ok`; proxy normalizes client behavior.

## Operator controls

- Start node with:

```bash
kexa-node \
  --network mainnet \
  --genesis /etc/kexa/genesis-mainnet.json \
  --rpc-addr 127.0.0.1:18040 \
  --p2p-addr 0.0.0.0:9040 \
  --data-dir /var/lib/kexa/mainnet \
  --peers "193.123.75.158:9040,141.145.159.171:9040"
```

- Host firewall policy:
  - allow inbound `9040/tcp`
  - deny inbound `18040/tcp`

Example checks:

```bash
ss -ltnp | grep -E ':9040|:18040'
curl -fsS http://127.0.0.1:18040/health
```

## Why `/peers/live` matters

- `/peers` is config intent.
- `/peers/live` is actual connected peers and is authoritative for networking truth.
