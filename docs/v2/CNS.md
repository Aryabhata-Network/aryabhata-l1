# Composite Node Score (CNS)

## Formula
CNS = (0.60 * U) + (0.25 * R) + (0.15 * P)
## Components

**U — Uptime Ratio**
Epochs online divided by last 100 epochs.
Range: 0.0 to 1.0

**R — Restart Stability**
R = max(0, 1 - restarts_per_100_epochs / 10)
10 or more restarts per 100 epochs = R score of 0.0

**P — Propagation Score**
P = max(0, 1 - median_delay / 180)
Median block propagation delay vs 180 second maximum.

---

## Status Thresholds

| CNS Range | Status | Effect |
|---|---|---|
| 0.80 to 1.00 | ELITE | Full rewards + 10% bonus |
| 0.60 to 0.79 | STANDARD | Full rewards |
| 0.40 to 0.59 | WARNED | No block submission |
| 0.20 to 0.39 | PENALISED | 24 epoch suspension |
| 0.00 to 0.19 | EXPELLED | Permanent ban |

---

## Anti-Gaming Properties

- CNS requires 100 epochs of history
- Cannot be manufactured quickly
- Sybil identities need 100+ hours honest operation
- Anchored to cumulative PoW history

---

## Fee Multiplier

| CNS | Multiplier |
|---|---|
| 0.80 and above | 0.80x — 20% discount |
| 0.60 and above | 1.00x — standard |
| 0.40 and above | 1.50x — surcharge |
| 0.20 and above | 3.00x — heavy surcharge |
| New address | 2.00x — new address premium |

---

*Version 2.0 — 2025 — Aryabhata Network*
