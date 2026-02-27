# 🌌 ARYABHATA LAYER-1 PROTOCOL

**Quantum-Resistant · Discipline-Driven · Mobile-Native**



![Status](https://img.shields.io/badge/Status-Pre--Genesis-gold?style=for-the-badge)




![Rust](https://img.shields.io/badge/Rust-52%25-orange?style=for-the-badge&logo=rust)




![Haskell](https://img.shields.io/badge/Haskell-48%25-purple?style=for-the-badge&logo=haskell)




![Mobile](https://img.shields.io/badge/Termux-Compatible-brightgreen?style=for-the-badge&logo=android)




![PostQuantum](https://img.shields.io/badge/Post--Quantum-Dilithium5-blue?style=for-the-badge)



> *"Mathematical Supremacy. Implementation Independence. Discipline Above All."*

---

## 📄 Whitepaper

| Version | Status | Link |
|---|---|---|
| **v2.0** | ✅ Current | [docs/v2/README.md](./docs/v2/README.md) |
| v1.0 | 🗄️ Archived | [docs/v1/README.md](./docs/v1/README.md) |

---

## 🧠 Architecture

**FORMAL PROOF LAYER**
Protocol invariants — immutable after genesis

**CONSENSUS ENGINE (Haskell)**
Pure, deterministic, no IO, no side-effects
SUNYA-PoW: Work + Time + Silence (CNS >= 0.60)

**EXECUTION SHELL (Rust)**
no_std — Stateless, memory-safe transport
Mobile-first: Android / Termux AArch64

---

## 🔐 Cryptographic Stack

| Layer | Algorithm | Purpose |
|---|---|---|
| Signatures | CRYSTALS-Dilithium5 (ML-DSA) | Quantum-resistant auth |
| KEM | CRYSTALS-Kyber1024 (ML-KEM) | P2P encryption |
| Hash | SHA3-512 / BLAKE3 | Block hash, Merkle tree |
| KDF | Argon2id | Wallet key derivation |
| PoW | SHA3-256 (SUNYA variant) | Mining puzzle |

---

## ⛏️ SUNYA-PoW — Three Conditions

`valid(B) = WORK(B) AND TIME(B) AND SILENCE(miner)`

| Condition | Rule |
|---|---|
| **WORK** | SHA3-256 hash meets target difficulty |
| **TIME** | Timestamp within +45s to +180s of parent block |
| **SILENCE** | Miner CNS score >= 0.60 |

---

## 📊 Key Parameters

| Parameter | Value |
|---|---|
| Total Supply | 14,000,000,000 ARY — fixed, immutable |
| Genesis Reward | 0.25 ARY per block |
| Target Block Time | 60 seconds |
| Difficulty Adjustment | Every 720 blocks |
| CNS Minimum to Mine | 0.60 |
| Mobile Binary Size | Less than 15 MB |
| RAM Active Mining | Less than 120 MB |

---

## 📱 Quick Start (Termux)

```bash
pkg update && pkg install rust clang git
git clone https://github.com/Aryabhata-Network/aryabhata-l1.git
cd aryabhata-l1/rust
cargo build --release
📂 Repository Structure
aryabhata-l1/
├── docs/
│   ├── v1/                  Archived whitepaper v1.0
│   └── v2/                  Current whitepaper v2.0
├── rust/src/
│   ├── network/             P2P transport layer
│   ├── mining/              SUNYA-PoW miner
│   ├── crypto/              Dilithium5, Kyber, SHA3
│   ├── ffi/                 Haskell FFI boundary
│   └── node/                Node lifecycle
├── haskell/src/Aryabhata/
│   ├── Consensus/           Pure state transitions
│   ├── Types/               Protocol types
│   └── Proof/               Property tests
├── genesis/                 Genesis block config
└── scripts/                 Helper scripts
⚠️ Disclaimer
Aryabhata (ARY) is not an investment product.
This protocol is provided AS-IS. Participation is at your own risk.
ARYABHATA NETWORK — 2025
