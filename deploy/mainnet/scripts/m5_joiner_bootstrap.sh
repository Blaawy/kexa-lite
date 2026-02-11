#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: m5_joiner_bootstrap.sh <env-file>

Artifact-only bootstrap for a mainnet joiner host.
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

LOCKED_MAINNET_GENESIS_HASH="${LOCKED_MAINNET_GENESIS_HASH:-692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5}"
LOCKED_TESTNET_GENESIS_HASH="${LOCKED_TESTNET_GENESIS_HASH:-1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159}"

: "${ARTIFACT_DIR:?missing ARTIFACT_DIR}"
: "${INSTALL_DIR:?missing INSTALL_DIR}"
: "${BIN_DIR:?missing BIN_DIR}"
: "${ETC_DIR:?missing ETC_DIR}"
: "${GENESIS_MAINNET_PATH:?missing GENESIS_MAINNET_PATH}"
: "${DATA_DIR_MAINNET:?missing DATA_DIR_MAINNET}"
: "${PROOF_ROOT:?missing PROOF_ROOT}"
: "${P2P_BIND_HOST:?missing P2P_BIND_HOST}"
: "${RPC_BIND_HOST:?missing RPC_BIND_HOST}"
: "${JOINER_P2P_PORT:?missing JOINER_P2P_PORT}"
: "${JOINER_RPC_PORT:?missing JOINER_RPC_PORT}"
: "${SEED_PEERS:?missing SEED_PEERS}"

if [[ "${NETWORK:-mainnet}" != "mainnet" ]]; then
  echo "FAIL: NETWORK must be mainnet" >&2
  exit 1
fi

TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
PROOF_DIR="$PROOF_ROOT/m5-joiner-${TIMESTAMP}"
mkdir -p "$PROOF_DIR" "$INSTALL_DIR" "$BIN_DIR" "$ETC_DIR" "$DATA_DIR_MAINNET"
mkdir -p "$INSTALL_DIR/release"

if [[ -d "$DATA_DIR_MAINNET" ]] && [[ -n "$(find "$DATA_DIR_MAINNET" -mindepth 1 -maxdepth 1 -print -quit 2>/dev/null || true)" ]]; then
  if [[ -f "$DATA_DIR_MAINNET/NETWORK.txt" && -f "$DATA_DIR_MAINNET/GENESIS_HASH.txt" ]]; then
    existing_network="$(<"$DATA_DIR_MAINNET/NETWORK.txt")"
    existing_hash="$(<"$DATA_DIR_MAINNET/GENESIS_HASH.txt")"
    if [[ "$existing_network" != "mainnet" || "$existing_hash" != "$LOCKED_MAINNET_GENESIS_HASH" ]]; then
      echo "FAIL: data-dir mismatch guard triggered for $DATA_DIR_MAINNET" >&2
      exit 1
    fi
  else
    echo "FAIL: data-dir non-empty but markers missing. Refusing ambiguous directory." >&2
    exit 1
  fi
fi

pushd "$ARTIFACT_DIR" >/dev/null

echo "CHECK 1: verifying SHA256SUMS"
sha256sum -c SHA256SUMS | tee "$PROOF_DIR/check1-sha256.txt"
echo "PASS CHECK 1: artifacts verified"

tar -xzf "kexa-node-0.1.0-rc1-x86_64-linux.tar.gz" -C "$INSTALL_DIR/release"
tar -xzf "kexa-cli-0.1.0-rc1-x86_64-linux.tar.gz" -C "$INSTALL_DIR/release"
NODE_BIN="$(find "$INSTALL_DIR/release" -type f -name kexa-node | head -n1)"
CLI_BIN="$(find "$INSTALL_DIR/release" -type f -name kexa-cli | head -n1)"

install -m 0755 "$NODE_BIN" "$BIN_DIR/kexa-node"
install -m 0755 "$CLI_BIN" "$BIN_DIR/kexa-cli"
install -m 0644 "genesis-mainnet.json" "$GENESIS_MAINNET_PATH"

popd >/dev/null

echo "CHECK 2: binary execution + genesis verification"
"$BIN_DIR/kexa-node" --network mainnet --genesis "$GENESIS_MAINNET_PATH" --print-genesis | tee "$PROOF_DIR/check2-mainnet-print-genesis.txt"
mainnet_hash="$(awk '/^genesis_hash:/ {print $2}' "$PROOF_DIR/check2-mainnet-print-genesis.txt")"
if [[ "$mainnet_hash" != "$LOCKED_MAINNET_GENESIS_HASH" ]]; then
  echo "FAIL: mainnet genesis hash mismatch: $mainnet_hash" >&2
  exit 1
fi
"$BIN_DIR/kexa-node" --network testnet --print-genesis | tee "$PROOF_DIR/check2-testnet-print-genesis.txt"
testnet_hash="$(awk '/^genesis_hash:/ {print $2}' "$PROOF_DIR/check2-testnet-print-genesis.txt")"
if [[ "$testnet_hash" != "$LOCKED_TESTNET_GENESIS_HASH" ]]; then
  echo "FAIL: testnet genesis hash mismatch: $testnet_hash" >&2
  exit 1
fi

echo "PASS CHECK 2: binary installed and genesis hashes locked"

cat > "$DATA_DIR_MAINNET/NETWORK.txt" <<NET
mainnet
NET
cat > "$DATA_DIR_MAINNET/GENESIS_HASH.txt" <<GH
$LOCKED_MAINNET_GENESIS_HASH
GH

RUN_CMD="$BIN_DIR/kexa-node --network mainnet --genesis $GENESIS_MAINNET_PATH --data-dir $DATA_DIR_MAINNET --rpc-addr ${RPC_BIND_HOST}:${JOINER_RPC_PORT} --p2p-addr ${P2P_BIND_HOST}:${JOINER_P2P_PORT} --peers ${SEED_PEERS}"
if [[ "$RUN_CMD" != *"--network mainnet"* ]]; then
  echo "FAIL: safety guard missing --network mainnet" >&2
  exit 1
fi

cat > "$PROOF_DIR/run-command.txt" <<CMD
$RUN_CMD
CMD

echo "PASS: bootstrap finished for joiner"
echo "PASS: locked mainnet genesis hash: $LOCKED_MAINNET_GENESIS_HASH"
echo "PASS: locked testnet genesis hash: $LOCKED_TESTNET_GENESIS_HASH"
