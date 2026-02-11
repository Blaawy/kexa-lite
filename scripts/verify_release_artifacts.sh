#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
DIST_DIR="${1:-$ROOT/dist}"
DIST_DIR="$(cd "$DIST_DIR" && pwd)"

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

[[ -d "$DIST_DIR" ]] || fail "dist directory not found: $DIST_DIR"
command -v sha256sum >/dev/null 2>&1 || fail "sha256sum is required"
command -v tar >/dev/null 2>&1 || fail "tar is required"

TESTNET_HASH="1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159"
MAINNET_HASH="692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5"

extract_hash() {
  sed -n 's/^genesis_hash: //p' | tail -n 1
}

TMP_DIR="$(mktemp -d)"
cleanup() { rm -rf "$TMP_DIR"; }
trap cleanup EXIT

cd "$DIST_DIR"
[[ -f SHA256SUMS ]] || fail "missing SHA256SUMS"

echo "[1/4] verifying sha256 checksums"
sha256sum -c SHA256SUMS

NODE_ARCHIVE="$(ls kexa-node-*-x86_64-linux.tar.gz 2>/dev/null | head -n 1)"
[[ -n "$NODE_ARCHIVE" ]] || fail "missing kexa-node release archive"

echo "[2/4] unpacking node binary"
tar -xzf "$NODE_ARCHIVE" -C "$TMP_DIR"
NODE_BIN="$(find "$TMP_DIR" -type f -name kexa-node | head -n 1)"
[[ -x "$NODE_BIN" ]] || fail "kexa-node binary not found after unpack"

echo "[3/4] checking locked testnet genesis hash"
TEST_HASH_GOT="$($NODE_BIN --network testnet --print-genesis | extract_hash)"
[[ "$TEST_HASH_GOT" == "$TESTNET_HASH" ]] || fail "testnet hash mismatch: got=$TEST_HASH_GOT expected=$TESTNET_HASH"

echo "[4/4] checking locked mainnet genesis hash"
[[ -f genesis-mainnet.json ]] || fail "missing genesis-mainnet.json"
MAIN_HASH_GOT="$($NODE_BIN --network mainnet --genesis "$DIST_DIR/genesis-mainnet.json" --print-genesis | extract_hash)"
[[ "$MAIN_HASH_GOT" == "$MAINNET_HASH" ]] || fail "mainnet hash mismatch: got=$MAIN_HASH_GOT expected=$MAINNET_HASH"

echo "PASS: release artifacts verified"
echo "  - dist: $DIST_DIR"
echo "  - testnet hash: $TEST_HASH_GOT"
echo "  - mainnet hash: $MAIN_HASH_GOT"
