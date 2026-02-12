Checkpoint: Feb 12, 2026 (Asia/Dubai) — CANONICAL HANDOFF v12

# TROUBLESHOOTING MAINNET

No guesswork. Use symptom → cause → exact fix.

## 1) Cannot connect to `:9040` from internet, cloud firewall is already open

**Symptom**
- `nc -vz <seed_ip> 9040` fails from external host.
- Node is running and cloud ingress/security-list appears correct.

**Root cause (observed in M5 dry-run)**
- Local `iptables` had REJECT rules blocking P2P before an allow rule.

**Exact fix**

```bash
sudo iptables -I INPUT 5 -p tcp --dport 9040 -j ACCEPT
sudo netfilter-persistent save
```

Then verify the allow is **before** a matching REJECT rule:

```bash
sudo iptables -L INPUT -n --line-numbers
```

If needed, re-insert ACCEPT at a smaller line number than the REJECT.

---

## 2) `/peers` shows entries but `/peers/live` is empty

**Cause**
- `/peers` lists configured seed targets only.
- `/peers/live` is real active sessions and remains authoritative.

**Fix**

```bash
curl -fsS http://127.0.0.1:18040/peers
curl -fsS http://127.0.0.1:18040/peers/live
ss -ltnp | grep 9040
```

- Confirm process is listening on `0.0.0.0:9040`.
- Confirm external reachability of seed IPs on `9040`.
- Keep checking `/peers/live` until non-empty.

---

## 3) Wrong network / wrong DB mismatch guard triggered

**Symptom**
- Node exits with fatal mismatch when switching network/genesis on an existing data dir.

**Cause**
- Existing DB metadata belongs to a different network/genesis.

**Fix**

```bash
# stop node first
rm -rf /var/lib/kexa/mainnet
mkdir -p /var/lib/kexa/mainnet
```

Restart with canonical mainnet flags only:

```bash
kexa-node \
  --network mainnet \
  --genesis /etc/kexa/genesis-mainnet.json \
  --rpc-addr 127.0.0.1:18040 \
  --p2p-addr 0.0.0.0:9040 \
  --data-dir /var/lib/kexa/mainnet \
  --peers "193.123.75.158:9040,141.145.159.171:9040"
```

---

## 4) RPC unreachable or exposed publicly

**Policy**
- RPC must remain private on `127.0.0.1:18040`.

**Fix**

```bash
# local health check
curl -fsS http://127.0.0.1:18040/health

# audit listening sockets
ss -ltnp | grep 18040
```

Expected bind: loopback only. If `0.0.0.0:18040` appears, stop and reconfigure immediately.

---

## 5) Genesis mismatch during verification

**Fix sequence**

```bash
sha256sum -c SHA256SUMS
kexa-node --network mainnet --genesis /etc/kexa/genesis-mainnet.json --print-genesis
```

Confirm exact mainnet hash:
- `692a347dab52762df864509bc9b0972408d9dc778ef0851190b18bb1555e1be5`

If mismatch persists, replace `genesis-mainnet.json` from verified release artifacts and repeat.
