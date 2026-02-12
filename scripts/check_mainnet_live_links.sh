#!/usr/bin/env bash
set -euo pipefail

CANONICAL_URL="https://github.com/Blaawy/kexa-lite/releases/tag/v0.1.0-rc1"

files=(
  "README.md"
  "docs/README.md"
  "docs/mainnet/MAINNET_LIVE.md"
  "kexa-explorer/app/verify/page.tsx"
)

missing=0
for file in "${files[@]}"; do
  if ! grep -Fq "$CANONICAL_URL" "$file"; then
    echo "Missing canonical Mainnet Live URL in $file"
    missing=1
  fi
done

if [[ "$missing" -ne 0 ]]; then
  exit 1
fi

echo "All required files contain canonical Mainnet Live URL."
