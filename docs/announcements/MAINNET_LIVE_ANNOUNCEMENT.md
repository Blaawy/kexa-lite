Checkpoint: Feb 12, 2026 (Asia/Dubai) — CANONICAL HANDOFF v12

# KEXA MAINNET LIVE — Release Announcement Draft

KEXA / KEXA-Lite v0 mainnet is now live under the canonical launch contract.

## Locked launch identities

- Mainnet genesis hash:
  `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
- Testnet frozen baseline hash:
  `1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159`
- Deterministic timestamp: `0`
- Reserve address:
  `kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv`

## Network entrypoints

- Seed1 P2P: `193.123.75.158:9040`
- Seed2 P2P: `141.145.159.171:9040`
- P2P port is public: `9040`
- RPC port is private localhost only: `127.0.0.1:18040`
- Explorer URL: `http://193.123.75.158/`

## How to verify in 60 seconds

```bash
# 1) Verify artifacts integrity
sha256sum -c SHA256SUMS

# 2) Verify deterministic mainnet genesis identity
kexa-node --network mainnet --genesis /etc/kexa/genesis-mainnet.json --print-genesis

# 3) Verify node endpoint sanity + live peering
curl -fsS http://127.0.0.1:18040/health && echo
curl -fsS http://127.0.0.1:18040/tip && echo
curl -fsS http://127.0.0.1:18040/peers/live && echo
```

Expected:
- all checksums `OK`
- genesis hash equals `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
- `/health` returns `ok`
- `/peers/live` is non-empty

## Economics and policy notes

- MAX_SUPPLY = `18,000,000 KEXA`
- SUBSIDY = `50 KEXA`
- Founder’s Reserve = `1.5% = 270,000 KEXA`
- Founder reserve is **policy-only in v0; consensus enforcement later.**

## Fair-launch doctrine

- No presale.
- No premine.
- No private sale.

## Canonical docs

- `docs/mainnet/MAINNET_LIVE.md`
- `docs/mainnet/JOIN_MAINNET.md`
- `docs/mainnet/VERIFY_MAINNET.md`
- `docs/mainnet/TROUBLESHOOTING_MAINNET.md`
- `docs/mainnet/SECURITY_MODEL.md`
