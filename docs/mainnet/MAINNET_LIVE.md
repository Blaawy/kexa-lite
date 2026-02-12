# MAINNET LIVE — KEXA v0.1.0-rc1

## Start Here (Canonical)

1. **Canonical release (first action):** https://github.com/Blaawy/kexa-lite/releases/tag/v0.1.0-rc1
2. **Explorer:** http://193.123.75.158/
3. **Seeds (P2P/public 9040):** `193.123.75.158:9040`, `141.145.159.171:9040`
4. **RPC (localhost/private 18040 only):** `127.0.0.1:18040`

> RPC must stay localhost-only. Public users should use the explorer proxy model and must not expose RPC directly.

## Locked Chain Identity

- **Mainnet genesis hash (locked):** `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
- **Testnet baseline hash (locked):** `1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159`

## Join Mainnet

Use release artifacts and release docs only (no source build required):
- Download artifact bundle + `SHA256SUMS` from the canonical release.
- Run checksum verification and genesis print verification from the release steps.
- Start node with public P2P `:9040` and private RPC `127.0.0.1:18040`.

Reference join workflow: [JOIN_MAINNET.md](./JOIN_MAINNET.md).

## Verify Contract

Use [VERIFY_MAINNET.md](./VERIFY_MAINNET.md) and treat `GET /peers/live` as networking truth.

You should verify:
- artifact integrity (`sha256sum -c SHA256SUMS`)
- locked genesis identity
- endpoint sanity (`/health`, `/tip`, `/peers/live`)

## Troubleshooting

If your node cannot be discovered, ensure firewall/iptables allows inbound **TCP 9040**.

See: [TROUBLESHOOTING_MAINNET.md](./TROUBLESHOOTING_MAINNET.md).
