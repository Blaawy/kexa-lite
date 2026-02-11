#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

TESTNET_HASH="1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159"
MAINNET_HASH="692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5"

[[ -x ./target/release/kexa-node ]] || {
  echo "ERROR: missing ./target/release/kexa-node; run scripts/release_build.sh first" >&2
  exit 1
}
[[ -f dist/genesis-mainnet.json ]] || {
  echo "ERROR: missing dist/genesis-mainnet.json; run scripts/release_build.sh first" >&2
  exit 1
}

extract_hash() {
  sed -n 's/^genesis_hash: //p' | tail -n 1
}

echo "[smoke] verifying testnet print-genesis"
TEST_HASH_GOT="$(./target/release/kexa-node --network testnet --print-genesis | extract_hash)"
[[ "$TEST_HASH_GOT" == "$TESTNET_HASH" ]] || {
  echo "ERROR: testnet hash mismatch: got=$TEST_HASH_GOT expected=$TESTNET_HASH" >&2
  exit 1
}

echo "[smoke] verifying mainnet print-genesis"
MAIN_HASH_GOT="$(./target/release/kexa-node --network mainnet --genesis dist/genesis-mainnet.json --print-genesis | extract_hash)"
[[ "$MAIN_HASH_GOT" == "$MAINNET_HASH" ]] || {
  echo "ERROR: mainnet hash mismatch: got=$MAIN_HASH_GOT expected=$MAINNET_HASH" >&2
  exit 1
}

echo "PASS: release smoketest succeeded"
