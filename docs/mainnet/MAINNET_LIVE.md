Checkpoint: Feb 12, 2026 (Asia/Dubai) — CANONICAL HANDOFF v12

# MAINNET LIVE — KEXA / KEXA-Lite v0

This is the canonical public launch bundle for KEXA mainnet activation.

## Status and doctrine (locked)

- Testnet baseline is **FROZEN/LOCKED**:
  - deterministic genesis timestamp = `0`
  - testnet genesis hash = `1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159`
  - rule: testnet remains baseline, no “nice to have” upgrades
- Mainnet engineering is **READY** (M1–M5 PASS). Engineering gates are complete; this bundle is for market activation.

## Mainnet constants (locked)

- Version: `0.1.0-rc1`
- MAX_SUPPLY = `18,000,000 KEXA`
- SUBSIDY = `50 KEXA`
- Founder’s Reserve = `1.5% = 270,000 KEXA` (v0 policy-only, not consensus-enforced yet)
- Mineable supply = `17,730,000`
- Mineable blocks = `354,600`
- Subsidy schedule:
  - `height 0 => 0`
  - `heights 1..=354,600 => 50`
  - `height >= 354,601 => 0 (fees only)`

## Mainnet genesis identity (locked)

- Reserve address (bech32):
  - `kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv`
- Mainnet genesis hash:
  - `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`

## Public network entrypoints

- Seed1 (P2P): `193.123.75.158:9040`
- Seed2 (P2P): `141.145.159.171:9040`
- RPC: `127.0.0.1:18040` only (private)
- Explorer: `http://193.123.75.158/`

> **Hard warning:** RPC is localhost-only; do not expose `18040` publicly.

## Official release artifacts (deterministic)

Artifacts are produced by `scripts/release_build.sh` and verified by:
- `scripts/release_smoketest.sh`
- `scripts/verify_release_artifacts.sh dist`

Expected `dist/` files:
- `kexa-node-0.1.0-rc1-x86_64-linux.tar.gz`
- `kexa-cli-0.1.0-rc1-x86_64-linux.tar.gz`
- `genesis-mainnet.json`
- `MAINNET_GENESIS.txt`
- `BUILD_MANIFEST.txt`
- `SHA256SUMS`

## 5-minute launch path (copy-paste)

### 1) Download artifacts

Use your preferred release source and place all files into one directory (example: `~/kexa-mainnet-rc1`).

```bash
mkdir -p ~/kexa-mainnet-rc1
cd ~/kexa-mainnet-rc1
# Put the six official dist files here.
```

### 2) Integrity verification

```bash
sha256sum -c SHA256SUMS
```

Expected: all files report `OK`.

### 3) Verify genesis identity

```bash
./kexa-node --network mainnet --genesis ./genesis-mainnet.json --print-genesis
```

Must show:
- genesis hash `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
- deterministic timestamp `0`
- reserve output includes `270000` to `kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv`

### 4) Start a mainnet node

```bash
./kexa-node \
  --network mainnet \
  --genesis /etc/kexa/genesis-mainnet.json \
  --rpc-addr 127.0.0.1:18040 \
  --p2p-addr 0.0.0.0:9040 \
  --data-dir /var/lib/kexa/mainnet \
  --peers "193.123.75.158:9040,141.145.159.171:9040"
```

### 5) Validate live connectivity

```bash
curl -fsS http://127.0.0.1:18040/health && echo
curl -fsS http://127.0.0.1:18040/tip | jq .
curl -fsS http://127.0.0.1:18040/peers | jq .
curl -fsS http://127.0.0.1:18040/peers/live | jq .
```

- `/peers` = configured peers from startup config.
- `/peers/live` = real active connections (**authoritative**).

Success condition: `/peers/live` is non-empty.

## Verification contract

Use the full verification contract: [VERIFY_MAINNET.md](./VERIFY_MAINNET.md)

It enforces all four checks:
- (A) artifact integrity (`SHA256SUMS`)
- (B) genesis identity (hash/timestamp/reserve)
- (C) networking truth (`/peers/live` non-empty and joinable externally)
- (D) endpoint sanity (`/health`, `/tip`, and mainnet RPC on `18040`, not `18030`)

## Join and operations docs

- External operator join guide: [JOIN_MAINNET.md](./JOIN_MAINNET.md)
- Troubleshooting (exact fixes): [TROUBLESHOOTING_MAINNET.md](./TROUBLESHOOTING_MAINNET.md)
- Security model (RPC-private + explorer proxy): [SECURITY_MODEL.md](./SECURITY_MODEL.md)
