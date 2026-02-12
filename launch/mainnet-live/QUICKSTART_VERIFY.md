Checkpoint: Feb 12, 2026 (Asia/Dubai) — CANONICAL HANDOFF v12

# QUICKSTART: Verify Mainnet

```bash
# A) integrity
sha256sum -c SHA256SUMS

# B) identity
kexa-node --network mainnet --genesis /etc/kexa/genesis-mainnet.json --print-genesis

# C + D) networking and endpoint sanity
curl -fsS http://127.0.0.1:18040/health && echo
curl -fsS http://127.0.0.1:18040/tip && echo
curl -fsS http://127.0.0.1:18040/peers && echo
curl -fsS http://127.0.0.1:18040/peers/live && echo
```

Pass conditions:
- all checksums `OK`
- `--print-genesis` output includes:
  - genesis hash `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
  - timestamp `0`
  - reserve address `kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv`
  - reserve amount `270000` (`270,000`)
- `/health` returns `ok`
- `/peers/live` is non-empty (authoritative connectivity)
