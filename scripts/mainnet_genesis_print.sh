#!/usr/bin/env bash
set -euo pipefail

OUT_FILE="${1:-mainnet_genesis_print.txt}"

cargo run -p kexa-node -- --network mainnet --genesis genesis/mainnet.json --print-genesis | tee "$OUT_FILE"
