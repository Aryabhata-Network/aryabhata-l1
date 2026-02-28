```markdown
# Aryabhata — Technical Architecture

## Layer Separation

Aryabhata enforces strict separation between three layers.
Each layer has exactly one responsibility.

---

## Layer 1 — Formal Proof Layer

**Responsibility:** Define protocol invariants.

All consensus rules are expressed as mathematical
properties and tested with QuickCheck.

Properties tested:
- CNS always in range 0.0 to 1.0
- Block reward never negative
- Supply never exceeds 14,000,000,000 ARY
- Penalty never makes CNS negative
- Double spend always results in permanent ban

---

## Layer 2 — Consensus Engine (Haskell)

**Responsibility:** Pure state transitions.

Rules:
- No IO
- No networking
- No system time
- No mutable state
- No exceptions

Every function is total and deterministic.
Same input always produces identical output.

Key modules:
- `Core.hs` — block validation
- `CNS.hs` — composite node score
- `Difficulty.hs` — difficulty adjustment
- `Supply.hs` — emission schedule
- `Penalty.hs` — automatic enforcement
- `State.hs` — protocol state transitions
- `Mempool.hs` — transaction priority
- `Checkpoint.hs` — long-range attack prevention
- `Fork.hs` — chain selection

---

## Layer 3 — Execution Shell (Rust)

**Responsibility:** Move bytes safely.

Rules:
- no_std
- forbid unsafe_code
- No consensus authority
- No cryptographic authority
- No state interpretation

Key modules:
- `crypto/` — Dilithium5, Kyber1024, SHA3
- `mining/` — SUNYA-PoW puzzle
- `network/` — P2P transport
- `ffi/` — Haskell boundary
- `node/` — lifecycle, config, storage

---

## FFI Safety Contract

All data crossing Haskell-Rust boundary:

| Rule | Specification |
|---|---|
| Format | [u32 length] + [u8 bytes] |
| Pointers | None — ever |
| Ownership | Rust owns all buffers |
| Timeout | 500ms hard deadline |
| Version | Must match — else reject |

---

## Mobile Architecture

Binary target: Android AArch64 via Termux

| Constraint | Value |
|---|---|
| Binary size | Less than 15 MB |
| RAM idle | Less than 40 MB |
| RAM mining | Less than 120 MB |
| Thermal limit | 42 degrees Celsius |
| Storage pruned | Less than 200 MB |

---

*Version 2.0 — 2025 — Aryabhata Network*
