# KEXA Mainnet Gate M1 — Decisions Locked (v0)

Checkpoint: Feb 8, 2026 (Asia/Dubai)  
Goal: MAINNET ONLY (testnet frozen baseline)

## Locked parameters (consensus-critical once wired in)

- Max supply: **18,000,000 KEXA**
- Subsidy per block: **50 KEXA**
- Founder’s Reserve concept: **1.5% = 270,000 KEXA**
  - **Decision (v0): policy-only** (no consensus-enforced lock/vesting in v0)
  - Lock/vesting pledge remains: **18m lock + 24m vest**
  - Enforcement/consensus design deferred to a later mainnet-track milestone

## Emission schedule (height-based)

- Height **0 (genesis)**: subsidy = **0**
- Heights **1..=354,600**: subsidy = **50**
- Heights **>= 354,601**: subsidy = **0** (fees only)

Supply math:

- Mineable supply = 18,000,000 − 270,000 = **17,730,000**
- Mineable blocks = 17,730,000 / 50 = **354,600**
- Total supply identity: mined (17,730,000) + reserve (270,000) = **18,000,000**

## Founder’s Reserve implementation note (genesis)

For v0 mainnet, the **270,000 KEXA reserve** is intended to be represented transparently as a **genesis allocation** to a published reserve address (address finalized at mainnet genesis lock step).  
Spending discipline (18m lock + 24m vest) is **policy-only** in v0.

