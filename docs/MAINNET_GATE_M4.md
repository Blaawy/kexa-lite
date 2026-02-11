# MAINNET Gate M4 — Release Candidate Discipline

## What M4 locks
M4 locks release process and verification discipline without changing consensus/economics.

The following identities remain locked and must not change:
- Testnet genesis hash: `1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159`
- Mainnet genesis hash: `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`
- Mainnet reserve amount/address:
  - `270000`
  - `kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv`

## Versioning strategy (pre-1.0 RC)
- Use SemVer pre-1.0 tags for launch hardening.
- Gate RC for this milestone: **`0.1.0-rc1`**.
- RC progression example:
  - `0.1.0-rc1`, `0.1.0-rc2`, ...
  - `0.1.0` only after launch-readiness sign-off.
- Every RC must update `CHANGELOG.md` and produce verifiable artifacts.

## Local release build (deterministic packaging)
From repo root:

```bash
scripts/release_build.sh
```

Outputs in `dist/`:
- `kexa-node-<version>-x86_64-linux.tar.gz`
- `kexa-cli-<version>-x86_64-linux.tar.gz`
- `genesis-mainnet.json`
- `MAINNET_GENESIS.txt`
- `BUILD_MANIFEST.txt`
- `SHA256SUMS`

Deterministic packaging details:
- `tar --sort=name --mtime=@$SOURCE_DATE_EPOCH --owner=0 --group=0 --numeric-owner`
- `gzip -n`

## Docker release build (recommended path)

```bash
scripts/release_docker_build.sh
```

This uses `docker/release.Dockerfile` with pinned Rust toolchain to reduce host variance.

## Verify release artifacts (single command)

```bash
scripts/verify_release_artifacts.sh dist
```

Verification includes:
1. `sha256sum -c SHA256SUMS`
2. `kexa-node --print-genesis --network testnet` matches locked testnet hash
3. `kexa-node --print-genesis --network mainnet --genesis dist/genesis-mainnet.json` matches locked mainnet hash

Expected result: final PASS summary and exit code `0`.
