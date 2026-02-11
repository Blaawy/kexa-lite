# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
with pre-1.0 release candidates for launch hardening.

## [0.1.0-rc1] - 2026-02-11

### Added
- M4 release engineering scripts:
  - `scripts/release_build.sh`
  - `scripts/release_smoketest.sh`
  - `scripts/verify_release_artifacts.sh`
  - `scripts/release_docker_build.sh`
- Docker release build recipe at `docker/release.Dockerfile`.
- `docs/MAINNET_GATE_M4.md` with RC process, build, and verification flow.
- `docs/UPGRADE_POLICY.md` documenting conservative upgrade boundaries.
- Deterministic `dist/` artifact generation with `SHA256SUMS` and build manifest.

### Changed
- Workspace and crate versions moved to `0.1.0-rc1` for RC discipline.
- `kexa-node --version` now exposes semantic version via clap metadata.
- Genesis JSON loading now strips optional UTF-8 BOM before parsing.

### Fixed
- Prevented BOM-related JSON parse failures for `genesis/mainnet.json` on Windows editors.
