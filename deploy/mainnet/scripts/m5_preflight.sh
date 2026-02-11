#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: m5_preflight.sh <env-file>

Runs preflight checks before artifact-only mainnet deployment.
USAGE
}

if [[ $# -ne 1 ]]; then
  usage
  exit 1
fi

ENV_FILE="$1"
if [[ ! -f "$ENV_FILE" ]]; then
  echo "FAIL: env file not found: $ENV_FILE" >&2
  exit 1
fi

# shellcheck disable=SC1090
source "$ENV_FILE"

: "${ARTIFACT_DIR:?missing ARTIFACT_DIR}"
: "${GENESIS_MAINNET_PATH:?missing GENESIS_MAINNET_PATH}"
: "${MAINNET_P2P_PORT:?missing MAINNET_P2P_PORT}"
: "${MAINNET_RPC_PORT:?missing MAINNET_RPC_PORT}"

required_cmds=(sha256sum tar curl awk sed grep ss)
missing=()
for cmd in "${required_cmds[@]}"; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    missing+=("$cmd")
  fi
done

if [[ ${#missing[@]} -gt 0 ]]; then
  echo "FAIL: missing required tools: ${missing[*]}" >&2
  exit 1
fi

echo "PASS: required tools present"

if [[ ! -d "$ARTIFACT_DIR" ]]; then
  echo "FAIL: ARTIFACT_DIR does not exist: $ARTIFACT_DIR" >&2
  exit 1
fi

echo "PASS: artifact directory exists: $ARTIFACT_DIR"

required_artifacts=(
  "$ARTIFACT_DIR/SHA256SUMS"
  "$ARTIFACT_DIR/kexa-node-0.1.0-rc1-x86_64-linux.tar.gz"
  "$ARTIFACT_DIR/kexa-cli-0.1.0-rc1-x86_64-linux.tar.gz"
  "$ARTIFACT_DIR/genesis-mainnet.json"
)
for artifact in "${required_artifacts[@]}"; do
  if [[ ! -f "$artifact" ]]; then
    echo "FAIL: missing artifact: $artifact" >&2
    exit 1
  fi
done

echo "PASS: expected artifacts are present"

if ss -ltn "sport = :$MAINNET_P2P_PORT" | grep -q LISTEN; then
  echo "FAIL: MAINNET_P2P_PORT already in use: $MAINNET_P2P_PORT" >&2
  exit 1
fi
if ss -ltn "sport = :$MAINNET_RPC_PORT" | grep -q LISTEN; then
  echo "FAIL: MAINNET_RPC_PORT already in use: $MAINNET_RPC_PORT" >&2
  exit 1
fi

echo "PASS: mainnet ports available (P2P=$MAINNET_P2P_PORT RPC=$MAINNET_RPC_PORT)"

echo "INFO: ensure firewall allows inbound tcp/$MAINNET_P2P_PORT and keeps RPC private (localhost or tunneled)."
