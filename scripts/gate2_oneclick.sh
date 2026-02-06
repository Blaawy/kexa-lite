#!/usr/bin/env bash
set -euo pipefail

NODE1="http://127.0.0.1:8030"
NODE2="http://127.0.0.1:8031"

FUND_BLOCKS=5
CONF_BLOCKS=1
AMOUNT=10
FEE=1

log() { echo "[$(date +%H:%M:%S)] $*"; }

wait_health() {
  local url="$1"
  for i in $(seq 1 60); do
    if curl -sf "$url/health" >/dev/null; then return 0; fi
    sleep 0.25
  done
  echo "ERROR: health not ready: $url" >&2
  return 1
}

tip() { curl -s "$1/tip" | tr -d '\r'; }

# Wait until both nodes report identical tip (sync), else fail.
must_match_tips() {
  local t1 t2
  for i in $(seq 1 120); do
    t1="$(tip "$NODE1")"
    t2="$(tip "$NODE2")"
    if [[ "$t1" == "$t2" ]]; then
      log "tips match: $t1"
      return 0
    fi
    sleep 0.25
  done
  echo "ERROR: tips mismatch (timeout waiting for node2 to sync)" >&2
  echo "node1: $t1" >&2
  echo "node2: $t2" >&2
  exit 1
}

balance() { curl -s "$1/balance/$2" | tr -d '\r'; }
utxos() { curl -s "$1/utxos/$2" | tr -d '\r'; }

post_mine() {
  local count="$1" addr="$2"
  local payload
  payload=$(printf '{"count":%s,"miner_address":"%s"}' "$count" "$addr")
  curl -s -X POST "$NODE1/mine_blocks" -H "Content-Type: application/json" -d "$payload" | tr -d '\r'
}

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root_dir"

log "docker: wipe devnet volumes"
docker compose down -v --remove-orphans >/dev/null 2>&1 || true

log "docker: up (build)"
docker compose up -d --build

log "wait: rpc health"
wait_health "$NODE1"
wait_health "$NODE2"
must_match_tips

log "build: wallet cli"
cargo build -p kexa-wallet >/dev/null

WALLET_BIN=""
if [[ -f "target/debug/kexa-wallet.exe" ]]; then WALLET_BIN="target/debug/kexa-wallet.exe"; fi
if [[ -f "target/debug/kexa-wallet" ]]; then WALLET_BIN="target/debug/kexa-wallet"; fi
if [[ -z "$WALLET_BIN" ]]; then echo "ERROR: wallet binary not found" >&2; exit 1; fi

# deterministic wallet names for repeatability
A_NAME="gate_alice"
B_NAME="gate_bob"
WALLET_DIR="$HOME/.kexa/wallets"
rm -f "$WALLET_DIR/${A_NAME}.json" "$WALLET_DIR/${B_NAME}.json" || true

log "wallet: create A/B"
"$WALLET_BIN" new "$A_NAME" >/dev/null
"$WALLET_BIN" new "$B_NAME" >/dev/null

A_ADDR="$("$WALLET_BIN" address "$A_NAME" | tr -d '\r')"
B_ADDR="$("$WALLET_BIN" address "$B_NAME" | tr -d '\r')"

log "addr A: $A_ADDR"
log "addr B: $B_ADDR"

log "mine: fund A ($FUND_BLOCKS blocks)"
post_mine "$FUND_BLOCKS" "$A_ADDR" | sed 's/^/[mine] /'
must_match_tips

A_BEFORE="$(balance "$NODE1" "$A_ADDR")"
B_BEFORE="$(balance "$NODE1" "$B_ADDR")"
log "balance before: A=$A_BEFORE B=$B_BEFORE"

log "tx: A -> B amount=$AMOUNT fee=$FEE"
TXID="$("$WALLET_BIN" send "$A_NAME" --to "$B_ADDR" --amount "$AMOUNT" --fee "$FEE" --node "$NODE1" | tr -d '\r')"
log "txid: $TXID"

if [[ ! "$TXID" =~ ^[0-9a-f]{64}$ ]]; then
  echo "ERROR: submit failed (expected 64-hex txid), got: $TXID" >&2
  exit 1
fi

log "mine: confirm ($CONF_BLOCKS blocks)"
post_mine "$CONF_BLOCKS" "$A_ADDR" | sed 's/^/[mine] /'
must_match_tips

A_AFTER="$(balance "$NODE1" "$A_ADDR")"
B_AFTER="$(balance "$NODE1" "$B_ADDR")"
log "balance after:  A=$A_AFTER B=$B_AFTER"

EXPECTED_B=$((B_BEFORE + AMOUNT))
if [[ "$B_AFTER" != "$EXPECTED_B" ]]; then
  echo "ERROR: bob balance wrong. expected=$EXPECTED_B got=$B_AFTER" >&2
  exit 1
fi

B_UTXOS="$(utxos "$NODE1" "$B_ADDR")"
echo "$B_UTXOS" | grep -q "$TXID" || { echo "ERROR: bob utxos missing txid" >&2; exit 1; }
echo "$B_UTXOS" | grep -q "\"amount\":$AMOUNT" || { echo "ERROR: bob utxos missing amount=$AMOUNT" >&2; exit 1; }

log "PASS: Gate2 one-click proof complete (wallet->tx->confirm)"
