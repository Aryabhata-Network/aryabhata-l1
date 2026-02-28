# SUNYA-PoW — Technical Specification

## Overview

SUNYA derives from Sanskrit meaning zero, void, origin.
SUNYA-PoW requires three independent conditions.
Hash power alone is insufficient.

---

## Validity Function
valid(B) = WORK(B) AND TIME(B) AND SILENCE(miner)
All three must be true simultaneously.
If any one fails, block is rejected.

---

## Condition 1 — WORK

SHA3-256 hash of block header must meet target difficulty.
SHA3-256(header || nonce) < target
Block header fields:
- Height (8 bytes)
- Parent hash (32 bytes)
- Merkle root (32 bytes)
- Timestamp (8 bytes)
- Nonce (8 bytes)

Total header: 88 bytes

---

## Condition 2 — TIME

Block timestamp must fall within window:
parent_timestamp + 45 <= timestamp <= parent_timestamp + 180
- Minimum gap: 45 seconds
- Maximum gap: 180 seconds
- Blocks outside window: protocol-invalid
- Prevents timestamp manipulation
- Prevents selfish mining

---

## Condition 3 — SILENCE

Miner CNS score must be at or above minimum:
CNS(miner) >= 0.60
CNS is computed from last 100 epochs.
New nodes need 100 epochs of honest operation.
Cannot be faked or purchased.

---

## Difficulty Adjustment

- Interval: every 720 blocks
- Target: 60 second block time
- Algorithm: exponential moving average
- Maximum change per period: plus or minus 25%
new_difficulty = current * cap(target_time / actual_avg_time)
cap range: 0.75 to 1.25
---

## Attack Resistance

| Attack | Defence |
|---|---|
| 51% | Needs hash AND CNS >= 0.60 |
| Selfish mining | TIME condition expires withheld blocks |
| Eclipse | Peer ASN diversity scoring |
| Long-range | Hardcoded checkpoints every 10,000 blocks |
| Spam blocks | SILENCE condition blocks new identities |
| Replay | Epoch-nonce per transaction |

---

*Version 2.0 — 2025 — Aryabhata Network*
