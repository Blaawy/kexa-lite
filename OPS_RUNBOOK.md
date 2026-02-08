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
