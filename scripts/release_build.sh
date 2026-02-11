#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(git rev-parse --show-toplevel)" && pwd -P)"
cd "$ROOT"

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

command -v cargo >/dev/null 2>&1 || fail "cargo is required"
command -v tar >/dev/null 2>&1 || fail "tar is required"
command -v gzip >/dev/null 2>&1 || fail "gzip is required"
command -v sha256sum >/dev/null 2>&1 || fail "sha256sum is required"
command -v git >/dev/null 2>&1 || fail "git is required"

export SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-0}"

VERSION="$(sed -n "s/^version = \"\(.*\)\"$/\1/p" crates/kexa-node/Cargo.toml | head -n 1)"
[[ -n "$VERSION" ]] || fail "failed to determine version"
TARGET="x86_64-unknown-linux-gnu"
DIST_DIR="$ROOT/dist"
BIN_DIR="$ROOT/target/release"

rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

echo "[1/6] building release binaries"
cargo build --release -p kexa-node -p kexa-cli

[[ -x "$BIN_DIR/kexa-node" ]] || fail "missing binary: $BIN_DIR/kexa-node"
[[ -x "$BIN_DIR/kexa-cli" ]] || fail "missing binary: $BIN_DIR/kexa-cli"

make_tar_gz() {
  local bin_name="$1"
  local out="$DIST_DIR/${bin_name}-${VERSION}-x86_64-linux.tar.gz"
  local stage
  stage="$(mktemp -d)"
  mkdir -p "$stage/${bin_name}-${VERSION}-x86_64-linux"
  cp "$BIN_DIR/$bin_name" "$stage/${bin_name}-${VERSION}-x86_64-linux/"

  (
    cd "$stage"
    tar --sort=name --mtime="@${SOURCE_DATE_EPOCH}" --owner=0 --group=0 --numeric-owner \
      -cf "${out%.gz}" "${bin_name}-${VERSION}-x86_64-linux"
  )
  gzip -n -f "${out%.gz}"
  rm -rf "$stage"
}

echo "[2/6] packaging deterministic tarballs"
make_tar_gz kexa-node
make_tar_gz kexa-cli

echo "[3/6] copying locked genesis package"
cp genesis/mainnet.json "$DIST_DIR/genesis-mainnet.json"
cat > "$DIST_DIR/MAINNET_GENESIS.txt" <<TXT
network: mainnet
genesis_hash: 692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5
reserve_amount: 270000
reserve_address: kexa1gxqcjr9vg2zsal3mj7ve7hfcy8np6sc4q430fphkzuqg88s5lhuslr34jv
invariants:
  - economics_locked: true
  - testnet_hash_locked: 1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159
TXT

echo "[4/6] writing build manifest"
RUSTC_VERSION="$(rustc --version)"
COMMIT_SHA="$(git rev-parse HEAD)"
cat > "$DIST_DIR/BUILD_MANIFEST.txt" <<TXT
version: ${VERSION}
target: ${TARGET}
rustc: ${RUSTC_VERSION}
commit: ${COMMIT_SHA}
source_date_epoch: ${SOURCE_DATE_EPOCH}
TXT

echo "[5/6] computing checksums"
(
  cd "$DIST_DIR"
  sha256sum \
    "kexa-node-${VERSION}-x86_64-linux.tar.gz" \
    "kexa-cli-${VERSION}-x86_64-linux.tar.gz" \
    genesis-mainnet.json \
    MAINNET_GENESIS.txt \
    BUILD_MANIFEST.txt > SHA256SUMS
)

echo "[6/6] release artifacts ready in $DIST_DIR"
ls -1 "$DIST_DIR"
