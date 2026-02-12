Checkpoint: Feb 12, 2026 (Asia/Dubai) — CANONICAL HANDOFF v12

# JOIN MAINNET (Artifact-Only)

This guide is for external operators joining KEXA mainnet from published release artifacts only.

## Locked inputs

- Version: `0.1.0-rc1`
- Mainnet genesis hash: `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
- Testnet genesis hash (frozen baseline): `1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159`
- Seeds:
  - `193.123.75.158:9040`
  - `141.145.159.171:9040`
- P2P = public on `9040`
- RPC = localhost-only on `127.0.0.1:18040`

## 1) Prepare host

```bash
sudo mkdir -p /opt/kexa/artifacts /opt/kexa/bin /etc/kexa /var/lib/kexa/mainnet
cd /opt/kexa/artifacts
```

Copy these files into `/opt/kexa/artifacts`:
- `kexa-node-0.1.0-rc1-x86_64-linux.tar.gz`
- `kexa-cli-0.1.0-rc1-x86_64-linux.tar.gz`
- `genesis-mainnet.json`
- `MAINNET_GENESIS.txt`
- `BUILD_MANIFEST.txt`
- `SHA256SUMS`

## 2) Verify integrity

```bash
cd /opt/kexa/artifacts
sha256sum -c SHA256SUMS
```

All lines must be `OK`.

## 3) Install binaries and genesis

```bash
cd /opt/kexa/artifacts
tar -xzf kexa-node-0.1.0-rc1-x86_64-linux.tar.gz
cp kexa-node /opt/kexa/bin/kexa-node
chmod +x /opt/kexa/bin/kexa-node

if [ -f kexa-cli-0.1.0-rc1-x86_64-linux.tar.gz ]; then
  tar -xzf kexa-cli-0.1.0-rc1-x86_64-linux.tar.gz || true
  [ -f kexa-cli ] && cp kexa-cli /opt/kexa/bin/kexa-cli && chmod +x /opt/kexa/bin/kexa-cli
fi

cp genesis-mainnet.json /etc/kexa/genesis-mainnet.json
```

## 4) Verify genesis identity before first run

```bash
/opt/kexa/bin/kexa-node --network mainnet --genesis /etc/kexa/genesis-mainnet.json --print-genesis
```

Confirm output contains:
- hash `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
- timestamp `0`
- reserve output `270000` to `kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv`

## 5) Start node (exact launch contract)

```bash
/opt/kexa/bin/kexa-node \
  --network mainnet \
  --genesis /etc/kexa/genesis-mainnet.json \
  --rpc-addr 127.0.0.1:18040 \
  --p2p-addr 0.0.0.0:9040 \
  --data-dir /var/lib/kexa/mainnet \
  --peers "193.123.75.158:9040,141.145.159.171:9040"
```

## 6) Verify it really joined

```bash
curl -fsS http://127.0.0.1:18040/health && echo
curl -fsS http://127.0.0.1:18040/tip && echo
curl -fsS http://127.0.0.1:18040/peers && echo
curl -fsS http://127.0.0.1:18040/peers/live && echo
```

Interpretation:
- `/peers`: configured peers from `--peers`.
- `/peers/live`: active TCP peers (**authoritative** live state).

Pass condition: `/peers/live` is non-empty.

## 7) Guardrails

- Never expose `18040` publicly.
- If you accidentally started on wrong network or wrong DB, stop node and follow [TROUBLESHOOTING_MAINNET.md](./TROUBLESHOOTING_MAINNET.md).
- Mainnet uses RPC `18040`; do not use testnet `18030`.
