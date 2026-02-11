#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: m5_collect_proof.sh <env-file> <seed1-rpc-url> <seed2-rpc-url> <joiner-rpc-url>

Collects and stores Gate M5 proof artifacts for all three nodes.
USAGE
}

if [[ $# -ne 4 ]]; then
  usage
  exit 1
fi

ENV_FILE="$1"
SEED1_RPC="$2"
SEED2_RPC="$3"
JOINER_RPC="$4"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "FAIL: env file not found: $ENV_FILE" >&2
  exit 1
fi

# shellcheck disable=SC1090
source "$ENV_FILE"

LOCKED_MAINNET_GENESIS_HASH="${LOCKED_MAINNET_GENESIS_HASH:-692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5}"
LOCKED_TESTNET_GENESIS_HASH="${LOCKED_TESTNET_GENESIS_HASH:-1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159}"
: "${PROOF_ROOT:?missing PROOF_ROOT}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERIFY_SCRIPT="$SCRIPT_DIR/m5_verify_rpc.sh"
if [[ ! -x "$VERIFY_SCRIPT" ]]; then
  echo "FAIL: verify script missing or not executable: $VERIFY_SCRIPT" >&2
  exit 1
fi

TS="$(date -u +%Y%m%dT%H%M%SZ)"
BUNDLE_DIR="$PROOF_ROOT/m5-proof-bundle-$TS"
mkdir -p "$BUNDLE_DIR"

echo "mainnet_genesis_hash=$LOCKED_MAINNET_GENESIS_HASH" | tee "$BUNDLE_DIR/genesis-locks.txt"
echo "testnet_genesis_hash=$LOCKED_TESTNET_GENESIS_HASH" | tee -a "$BUNDLE_DIR/genesis-locks.txt"

echo "PASS: locked hashes recorded"

"$VERIFY_SCRIPT" "$SEED1_RPC" "$LOCKED_MAINNET_GENESIS_HASH" "$BUNDLE_DIR/seed1"
"$VERIFY_SCRIPT" "$SEED2_RPC" "$LOCKED_MAINNET_GENESIS_HASH" "$BUNDLE_DIR/seed2"
"$VERIFY_SCRIPT" "$JOINER_RPC" "$LOCKED_MAINNET_GENESIS_HASH" "$BUNDLE_DIR/joiner"

echo "PASS: collected RPC proofs for seed1/seed2/joiner"

# Cross-proof snapshots
curl -fsS "$SEED1_RPC/peers/live" | tee "$BUNDLE_DIR/seed1-peers-live.json" >/dev/null
curl -fsS "$SEED2_RPC/peers/live" | tee "$BUNDLE_DIR/seed2-peers-live.json" >/dev/null
curl -fsS "$JOINER_RPC/peers/live" | tee "$BUNDLE_DIR/joiner-peers-live.json" >/dev/null

echo "PASS: peers/live snapshots captured"
echo "PASS: proof bundle at $BUNDLE_DIR"
