#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: m5_verify_rpc.sh <rpc-base-url> <expected-mainnet-genesis-hash> [proof-dir]
Example: m5_verify_rpc.sh http://127.0.0.1:18040 692a347d... ./proof
USAGE
}

if [[ $# -lt 2 || $# -gt 3 ]]; then
  usage
  exit 1
fi

RPC_BASE="$1"
EXPECTED_GENESIS="$2"
PROOF_DIR="${3:-./proof}"
mkdir -p "$PROOF_DIR"

health_out="$PROOF_DIR/health.txt"
tip_out="$PROOF_DIR/tip.json"
blocks_out="$PROOF_DIR/blocks_limit1.json"
peers_out="$PROOF_DIR/peers_live.json"

curl -fsS "$RPC_BASE/health" | tee "$health_out" >/dev/null
if ! grep -q 'ok' "$health_out"; then
  echo "FAIL CHECK 3: /health not ok" >&2
  exit 1
fi

echo "PASS CHECK 3.1: /health ok"

curl -fsS "$RPC_BASE/tip" | tee "$tip_out" >/dev/null
if ! grep -q '"height"' "$tip_out" || ! grep -q '"hash"' "$tip_out"; then
  echo "FAIL CHECK 3: /tip missing height/hash" >&2
  exit 1
fi

echo "PASS CHECK 3.2: /tip includes height/hash"

curl -fsS "$RPC_BASE/blocks?limit=1" | tee "$blocks_out" >/dev/null
if ! grep -q "$EXPECTED_GENESIS" "$blocks_out"; then
  echo "FAIL CHECK 3: genesis hash not found in /blocks?limit=1" >&2
  exit 1
fi

echo "PASS CHECK 3.3: /blocks?limit=1 includes locked genesis hash"

curl -fsS "$RPC_BASE/peers/live" | tee "$peers_out" >/dev/null
if grep -qE '^\[\s*\]$' "$peers_out" || grep -qE '^\{\s*"peers"\s*:\s*\[\s*\]\s*\}$' "$peers_out"; then
  echo "FAIL CHECK 3: /peers/live is empty" >&2
  exit 1
fi

echo "PASS CHECK 3.4: /peers/live is non-empty"
