#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: m5_seed_bootstrap.sh <env-file> <seed1|seed2>

Artifact-only bootstrap for a mainnet seed host.
USAGE
}

if [[ $# -ne 2 ]]; then
  usage
  exit 1
fi

ENV_FILE="$1"
SEED_NAME="$2"
if [[ "$SEED_NAME" != "seed1" && "$SEED_NAME" != "seed2" ]]; then
  echo "FAIL: seed name must be seed1 or seed2" >&2
  exit 1
fi
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
: "${LOG_DIR:?missing LOG_DIR}"
: "${PROOF_ROOT:?missing PROOF_ROOT}"
: "${P2P_BIND_HOST:?missing P2P_BIND_HOST}"
: "${RPC_BIND_HOST:?missing RPC_BIND_HOST}"
: "${SEED1_PUBLIC_P2P_IP:?missing SEED1_PUBLIC_P2P_IP}"
: "${SEED2_PUBLIC_P2P_IP:?missing SEED2_PUBLIC_P2P_IP}"
: "${SEED1_P2P_PORT:?missing SEED1_P2P_PORT}"
: "${SEED2_P2P_PORT:?missing SEED2_P2P_PORT}"
: "${SEED1_RPC_PORT:?missing SEED1_RPC_PORT}"
: "${SEED2_RPC_PORT:?missing SEED2_RPC_PORT}"

TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
PROOF_DIR="$PROOF_ROOT/m5-${SEED_NAME}-${TIMESTAMP}"
mkdir -p "$PROOF_DIR" "$INSTALL_DIR" "$BIN_DIR" "$ETC_DIR" "$DATA_DIR_MAINNET" "$LOG_DIR"

if [[ "${NETWORK:-mainnet}" != "mainnet" ]]; then
  echo "FAIL: NETWORK must be mainnet" >&2
  exit 1
fi

if [[ -d "$DATA_DIR_MAINNET" ]] && [[ -n "$(find "$DATA_DIR_MAINNET" -mindepth 1 -maxdepth 1 -print -quit 2>/dev/null || true)" ]]; then
  if [[ -f "$DATA_DIR_MAINNET/NETWORK.txt" && -f "$DATA_DIR_MAINNET/GENESIS_HASH.txt" ]]; then
    existing_network="$(<"$DATA_DIR_MAINNET/NETWORK.txt")"
    existing_hash="$(<"$DATA_DIR_MAINNET/GENESIS_HASH.txt")"
    if [[ "$existing_network" != "mainnet" || "$existing_hash" != "$LOCKED_MAINNET_GENESIS_HASH" ]]; then
      echo "FAIL: data-dir mismatch guard triggered for $DATA_DIR_MAINNET" >&2
      echo "expected network=mainnet genesis=$LOCKED_MAINNET_GENESIS_HASH" >&2
      echo "found network=$existing_network genesis=$existing_hash" >&2
      exit 1
    fi
  else
    echo "FAIL: data-dir is non-empty but missing NETWORK.txt/GENESIS_HASH.txt markers: $DATA_DIR_MAINNET" >&2
    echo "Refusing to reuse ambiguous data dir. Back it up or wipe it intentionally." >&2
    exit 1
  fi
fi

pushd "$ARTIFACT_DIR" >/dev/null

echo "CHECK 1: verifying SHA256SUMS"
sha256sum -c SHA256SUMS | tee "$PROOF_DIR/check1-sha256.txt"
echo "PASS CHECK 1: artifacts verified"

mkdir -p "$INSTALL_DIR/release"
tar -xzf "kexa-node-0.1.0-rc1-x86_64-linux.tar.gz" -C "$INSTALL_DIR/release"
tar -xzf "kexa-cli-0.1.0-rc1-x86_64-linux.tar.gz" -C "$INSTALL_DIR/release"

NODE_BIN="$(find "$INSTALL_DIR/release" -type f -name kexa-node | head -n1)"
CLI_BIN="$(find "$INSTALL_DIR/release" -type f -name kexa-cli | head -n1)"

if [[ -z "$NODE_BIN" || -z "$CLI_BIN" ]]; then
  echo "FAIL: unable to locate extracted binaries under $INSTALL_DIR/release" >&2
  exit 1
fi

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

if [[ "$SEED_NAME" == "seed1" ]]; then
  P2P_PORT="$SEED1_P2P_PORT"
  RPC_PORT="$SEED1_RPC_PORT"
  PEERS="${SEED2_PUBLIC_P2P_IP}:${SEED2_P2P_PORT}"
else
  P2P_PORT="$SEED2_P2P_PORT"
  RPC_PORT="$SEED2_RPC_PORT"
  PEERS="${SEED1_PUBLIC_P2P_IP}:${SEED1_P2P_PORT}"
fi

cat > "$DATA_DIR_MAINNET/NETWORK.txt" <<NET
mainnet
NET
cat > "$DATA_DIR_MAINNET/GENESIS_HASH.txt" <<GH
$LOCKED_MAINNET_GENESIS_HASH
GH

RUN_CMD="$BIN_DIR/kexa-node --network mainnet --genesis $GENESIS_MAINNET_PATH --data-dir $DATA_DIR_MAINNET --rpc-addr ${RPC_BIND_HOST}:${RPC_PORT} --p2p-addr ${P2P_BIND_HOST}:${P2P_PORT} --peers ${PEERS}"
if [[ "$RUN_CMD" != *"--network mainnet"* ]]; then
  echo "FAIL: safety guard missing --network mainnet" >&2
  exit 1
fi

cat > "$PROOF_DIR/run-command.txt" <<CMD
$RUN_CMD
CMD

echo "PASS: bootstrap finished for $SEED_NAME"
echo "PASS: locked mainnet genesis hash: $LOCKED_MAINNET_GENESIS_HASH"
echo "PASS: locked testnet genesis hash: $LOCKED_TESTNET_GENESIS_HASH"
echo "NEXT: start node with command in $PROOF_DIR/run-command.txt"
