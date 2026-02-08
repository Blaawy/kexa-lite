# KEXA / KEXA-Lite v0 — Testnet

## Seed
- P2P: 193.123.75.158:9030,141.145.159.171:9030

## Default ports
- P2P (public): 9030
- RPC (local): 8030

## Chain identity (genesis)
Genesis block hash (height 0):
- 1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159

Expected genesis properties:
- timestamp: 0
- bits: 16
- coinbase subsidy: 50 to zero-address

## Join testnet (example)
Run a node:
```bash
kexa-node \
  --rpc-addr 127.0.0.1:8030 \
  --p2p-addr 0.0.0.0:9030 \
  --data-dir ./kexa-testnet-data \
  --peers "193.123.75.158:9030,141.145.159.171:9030"
```

## Verify you’re on the right chain
```bash
curl -s http://127.0.0.1:8030/block/1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159
```

## Local status helper
```bash
bash scripts/testnet_status.sh
```
## Mini-Explorer (CLI)

After your node is running (RPC is local), you can browse the chain without guessing hashes:

```bash
# build kexa-cli (no Rust installed)
docker run --rm -v "$PWD":/app -w /app rust:1.85 bash -c "cargo build -p kexa-cli --release"

# browse via your local RPC (example: 127.0.0.1:8030)
./target/release/kexa-cli --rpc http://127.0.0.1:8030 health
./target/release/kexa-cli --rpc http://127.0.0.1:8030 tip
./target/release/kexa-cli --rpc http://127.0.0.1:8030 blocks --last 20
./target/release/kexa-cli --rpc http://127.0.0.1:8030 block --height 0
```

RPC endpoints used:
- `GET /blocks?limit=N`
- `GET /block/:hash`


## Windows quick verify

Seed (P2P): `193.123.75.158:9030,141.145.159.171:9030`  
Local RPC default: `http://127.0.0.1:8030`

Run:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\local_status.ps1
powershell -ExecutionPolicy Bypass -File .\scripts\join_verify_testnet.ps1
```

## Seed VPS (ops notes)

- Seed runs as non-root inside Docker: `user: "1000:1000"`
- VPS uses a systemd `ExecStartPre` perms fix (`/usr/local/bin/kexa-fix-perms`) so the sled DB stays writable across restarts.
- RPC stays private: `127.0.0.1:18030` (host) → `8030` (container), and P2P is public on `9030`.


## If you get a FAIL alert (Seed monitoring)

**FAIL means at least one of these happened:**
- Seed service is down (`kexa-seed.service` or `kexa-seed2.service` not active)
- RPC is not responding (`127.0.0.1:18030` on Seed1 / `127.0.0.1:18031` on Seed2)
- Disk is high (space or inodes)
- `/tip` is missing or not parseable
- `peers/live == 0` sustained (default window, e.g. ~5 minutes)

### One command to collect diagnostics (run on the seed VPS)

SSH into the seed VPS, then run:

```bash
sudo bash -lc '
set -euo pipefail
echo "===== KEXA FAIL DIAG ====="
echo "HOST: $(hostname)"; date; echo

echo "== UNITS (seed/monitor) =="
systemctl list-units --type=service --all | grep -E "kexa-(seed|monitor)\.service" || true
echo

echo "== TIMERS =="
systemctl list-timers --all | grep -i kexa-monitor || true
echo

echo "== kexa-monitor.service (wiring) =="
systemctl cat kexa-monitor.service || true
echo

echo "== LAST 50 monitor logs =="
journalctl -u kexa-monitor.service -n 50 --no-pager || true
echo

RPC=""
for p in 18030 18031 8030; do
  if curl -fsS --max-time 2 "http://127.0.0.1:${p}/health" >/dev/null 2>&1; then RPC="http://127.0.0.1:${p}"; break; fi
done
echo "== RPC BASE =="; echo "${RPC:-NOT_FOUND}"; echo

if [ -n "${RPC}" ]; then
  echo "== RPC /health =="; curl -sS --max-time 3 "$RPC/health"; echo; echo
  echo "== RPC /tip =="; curl -sS --max-time 3 "$RPC/tip"; echo; echo
  echo "== RPC /peers/live =="; curl -sS --max-time 3 "$RPC/peers/live"; echo; echo
fi

echo "== PORTS (9030 + RPC) =="
ss -lntp | grep -E ":9030|:18030|:18031|:8030" || true
echo

echo "== DISK (space + inodes) =="
df -h /; echo; df -i /
echo "===== END ====="
'


### Where to look (logs) + force a run

On the seed VPS (SSH), use:

```bash
# monitor logs (most important)
sudo journalctl -u kexa-monitor.service -n 80 --no-pager

# seed logs (run the one that exists on that machine)
sudo journalctl -u kexa-seed.service -n 120 --no-pager || true
sudo journalctl -u kexa-seed2.service -n 120 --no-pager || true

# force monitor run now (and ping Healthchecks)
sudo systemctl start kexa-monitor.service

```
