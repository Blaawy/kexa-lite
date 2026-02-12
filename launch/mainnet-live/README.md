Checkpoint: Feb 12, 2026 (Asia/Dubai) — CANONICAL HANDOFF v12

# KEXA Mainnet Live Pack (Docs + Examples)

This pack contains only documentation and safe templates.
It does **not** include binaries and does not modify release artifacts.

## Use this pack with official rc1 artifacts

Expected artifacts:
- `kexa-node-0.1.0-rc1-x86_64-linux.tar.gz`
- `kexa-cli-0.1.0-rc1-x86_64-linux.tar.gz`
- `genesis-mainnet.json`
- `MAINNET_GENESIS.txt`
- `BUILD_MANIFEST.txt`
- `SHA256SUMS`

## Verification contract summary

1. Integrity: `sha256sum -c SHA256SUMS`
2. Identity: `kexa-node --network mainnet --genesis /etc/kexa/genesis-mainnet.json --print-genesis`
3. Networking: `/peers/live` must be non-empty
4. Endpoint sanity: `/health` returns `ok`; `/tip` returns hash/height; RPC is localhost `18040`

## Canonical docs

- `../../docs/mainnet/MAINNET_LIVE.md`
- `../../docs/mainnet/JOIN_MAINNET.md`
- `../../docs/mainnet/VERIFY_MAINNET.md`
- `../../docs/mainnet/TROUBLESHOOTING_MAINNET.md`
- `../../docs/mainnet/SECURITY_MODEL.md`

## Examples

- `./QUICKSTART_JOIN.md`
- `./QUICKSTART_VERIFY.md`
- `./examples/systemd.example.service`
- `./examples/env.example`
