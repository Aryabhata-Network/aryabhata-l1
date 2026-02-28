# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| 2.0 | YES |
| 1.0 | NO — archived |

---

## Threat Model

Aryabhata is designed to resist three threat classes:

**Class 1 — Classical Attacks**
51% attack, eclipse attack, selfish mining,
long-range attack, replay attack, double spend.

**Class 2 — Quantum Attacks**
Shor algorithm against signatures,
Grover algorithm against hash functions.

**Class 3 — AI Adversarial Attacks**
Traffic analysis, timing analysis,
model-based key extraction, pattern recognition.

---

## Security Properties

| Property | Implementation |
|---|---|
| Post-quantum signatures | CRYSTALS-Dilithium5 (ML-DSA) |
| Post-quantum encryption | CRYSTALS-Kyber1024 (ML-KEM) |
| Quantum-safe hashing | SHA3-512 / BLAKE3 |
| Memory safety | Rust borrow checker + zeroize |
| No unsafe code | `#![forbid(unsafe_code)]` |
| Timing attack prevention | Constant-time comparisons |
| Traffic analysis prevention | Fixed-frame messages |
| Replay attack prevention | Epoch-nonce per transaction |
| Eclipse attack prevention | Peer ASN diversity scoring |
| Long-range attack prevention | Hardcoded checkpoints |

---

## Reporting a Vulnerability

If you find a security vulnerability:

1. Do NOT open a public GitHub issue
2. Email: security@aryabhata.network
3. Include full description and reproduction steps
4. Allow 90 days for fix before public disclosure

---

## Out of Scope

- Bugs in third-party dependencies
- Social engineering attacks
- Physical device attacks

---

*Aryabhata Network — 2025*
