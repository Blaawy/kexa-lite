#!/usr/bin/env bash
set -euo pipefail

SEED_IP="193.123.75.158"
SEED_P2P_PORT="9030"
GENESIS="1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159"
LOCAL_RPC="${LOCAL_RPC:-http://127.0.0.1:8030}"

echo "== Seed P2P (should be reachable) =="
( echo >"/dev/tcp/${SEED_IP}/${SEED_P2P_PORT}" ) >/dev/null 2>&1 \
  && echo "OK: ${SEED_IP}:${SEED_P2P_PORT} reachable" \
  || echo "WARN: /dev/tcp not supported here (or port blocked)"

echo
echo "== Local node RPC =="
curl -s "${LOCAL_RPC}/health" && echo
curl -s "${LOCAL_RPC}/tip" && echo

echo
echo "== Genesis identity check (local RPC) =="
curl -s "${LOCAL_RPC}/block/${GENESIS}" && echo
