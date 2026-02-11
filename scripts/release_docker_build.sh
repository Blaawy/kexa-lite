#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

command -v docker >/dev/null 2>&1 || {
  echo "ERROR: docker is required for containerized release build" >&2
  exit 1
}

IMAGE="kexa-release-build:0.1.0-rc1"

echo "[docker] building release image"
docker build -f docker/release.Dockerfile -t "$IMAGE" .

echo "[docker] running deterministic release build"
docker run --rm \
  -e SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-0}" \
  -v "$ROOT":/work \
  -w /work \
  "$IMAGE" \
  "scripts/release_build.sh"

echo "[docker] done: dist artifacts are in $ROOT/dist"
