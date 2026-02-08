# KEXA Testnet Ops Runbook (Seeds + Monitoring)

Safe-to-share: contains no secrets. Do NOT paste Healthchecks URLs anywhere.

Seed1: P2P 193.123.75.158:9030 | RPC http://127.0.0.1:18030 | unit kexa-seed.service
Seed2: P2P 141.145.159.171:9030 | RPC http://127.0.0.1:18031 | unit kexa-seed2.service

Truth order when HC is red:
1) journalctl -u kexa-monitor.service -n 80
2) docker ps + docker inspect (container status + StartedAt)
3) curl /health + /tip + /peers/live
4) docker logs --tail 200
5) restart seed unit if needed, re-check

## Notes

- Seed2 defaults were fixed: /usr/local/bin/kexa-monitor now defaults to kexa-seed2.service + 127.0.0.1:18031.

<!-- BEGIN_V8_INCIDENT_RESPONSE_ADDENDUM -->
## v8 Incident Response (Fast Path)

**Truth rule:** `kexa-seed*.service` is oneshot → systemd can look “active” while the container is dead. **Truth = Docker + RPC.**

**When Healthchecks is red:**
- Run: `ks --strict`
- If FAIL, paste back (evidence bundle):
  - `ks --strict` output
  - `journalctl -u kexa-monitor.service -n 20 --no-pager`
  - `docker ps | grep -i kexa`
  - `docker logs --tail=120 <container_name>`

**Restart (seed-specific):**
- Seed1: `sudo systemctl restart kexa-seed.service`
- Seed2: `sudo systemctl restart kexa-seed2.service`
Then re-run: `ks --strict`
<!-- END_V8_INCIDENT_RESPONSE_ADDENDUM -->

<!-- BEGIN_V8_LOG_RETENTION_POLICY -->
## v8 Ops Addendum — Log Retention (journald)

Goal: keep enough history for incidents without filling disk.

Recommended policy:
- Vacuum size: `journalctl --vacuum-size=200M`
- Vacuum time: `journalctl --vacuum-time=14d`

Optional (persist across reboots): set in `/etc/systemd/journald.conf`:
- `SystemMaxUse=200M`
- `SystemMaxFileSize=50M`
Then: `sudo systemctl restart systemd-journald`
<!-- END_V8_LOG_RETENTION_POLICY -->
