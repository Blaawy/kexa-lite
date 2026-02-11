# KEXA-Lite v0 Upgrade Policy (Conservative)

## Principles
- Mainnet launch safety is prioritized over feature velocity.
- Prefer policy/docs/ops updates over consensus changes whenever possible.
- Minimize post-launch binary churn; batch non-critical fixes.

## Change classes

### 1) New binary only (no consensus impact)
These changes can be shipped as normal software updates:
- RPC, CLI UX, diagnostics, logging, docs
- Release scripts, packaging, checksum workflows
- Non-consensus bug fixes in node internals

Operators may rolling-upgrade binaries if network compatibility is preserved.

### 2) Hard fork / consensus change
Any change that affects block/tx validity or chain state transitions requires coordinated network upgrade:
- Emission schedule, subsidy, maturity, PoW validation semantics
- Transaction validity rules/signature/serialization consensus behavior
- Header/block validation logic that changes acceptance criteria

These are protocol changes and require explicit governance/coordination.

### 3) Re-genesis (launch reset event; almost never)
Genesis changes are exceptional and treated as launch reset events:
- Height-0 contents or hash changes
- Reserve allocation/address changes in genesis
- Header genesis fields affecting chain identity

Re-genesis is not an in-place upgrade. It creates a distinct network identity.

## v0 posture
- Keep testnet baseline frozen unless needed for mainnet launch safety.
- Keep mainnet genesis hash and economics locked.
- Default to additive operational hardening, not consensus drift.
