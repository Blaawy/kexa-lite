Checkpoint: Feb 12, 2026 (Asia/Dubai) — CANONICAL HANDOFF v12

# VERIFY MAINNET — Integrity + Identity + Networking Contract

This contract is the minimum proof set for a valid KEXA mainnet launch/join.

## Locked truth set

- Mainnet hash: `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
- Testnet frozen baseline hash: `1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159`
- Deterministic timestamp: `0`
- Reserve address: `kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv`
- Reserve amount: `270000` (1.5% policy reserve in v0)
- Mainnet RPC port: `18040` (localhost only)
- Mainnet P2P port: `9040` (public)

---

## (A) Artifact integrity

Run in artifact directory:

```bash
sha256sum -c SHA256SUMS
```

Pass criteria:
- every listed artifact prints `OK`
- no missing files

If this fails: stop. Do not launch.

---

## (B) Genesis identity

Print deterministic genesis:

```bash
kexa-node --network mainnet --genesis /etc/kexa/genesis-mainnet.json --print-genesis
```

Expected (mechanical reserve-output check):

Confirm these fields/values appear in the output:
- genesis hash = `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
- timestamp = `0`
- reserve address = `kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv`
- reserve output amount = `270000` (equivalent to `270,000`)

Optional frozen testnet baseline check:

```bash
kexa-node --network testnet --print-genesis
```

Must show testnet hash `1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159`.

---

## (C) Networking truth

Start node with the canonical contract:

```bash
kexa-node \
  --network mainnet \
  --genesis /etc/kexa/genesis-mainnet.json \
  --rpc-addr 127.0.0.1:18040 \
  --p2p-addr 0.0.0.0:9040 \
  --data-dir /var/lib/kexa/mainnet \
  --peers "193.123.75.158:9040,141.145.159.171:9040"
```

Query peer endpoints:

```bash
curl -fsS http://127.0.0.1:18040/peers && echo
curl -fsS http://127.0.0.1:18040/peers/live && echo
```

Interpretation (critical):
- `/peers` = configured peers only.
- `/peers/live` = real connected peers (**authoritative**).

Pass criteria:
- `/peers/live` is non-empty.
- At least one external check confirms TCP connect to seed P2P `:9040`.

External joinability probe example:

```bash
nc -vz 193.123.75.158 9040
nc -vz 141.145.159.171 9040
```

---

## (D) Endpoint sanity

```bash
curl -fsS http://127.0.0.1:18040/health && echo
curl -fsS http://127.0.0.1:18040/tip && echo
curl -fsS "http://127.0.0.1:18040/blocks?limit=1" && echo
```

Pass criteria:
- `/health` returns plain text `ok`
- `/tip` returns JSON including `height` and `hash`
- endpoint is mainnet RPC `18040`, not `18030`

---

## Verification outcome

A node is considered verified only when **A + B + C + D all pass**.
