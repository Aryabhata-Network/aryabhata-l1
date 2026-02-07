# 🌌 ARYABHATA LAYER-1 PROTOCOL
**Status: Core Frozen — Phase: Pre-Genesis**

Aryabhata is a mathematically defined, post-quantum resistant Layer-1 blockchain
protocol designed for long-term immutability (50 years).

The system enforces a strict separation between:

- **Consensus Truth** (Haskell + Formal Proofs)
- **Execution & Transport** (Rust, stateless, no authority)

After Genesis, any modification to protocol laws or architectural boundaries
defines a *different protocol*, not an upgrade.

---

## 🧠 Core Design Philosophy

### 1. Mathematical Supremacy
Consensus rules are defined as pure state-transition laws and formally proven.  
If code, documentation, or social agreement conflicts with proofs,  
**the proofs are authoritative.**

---

### 2. Quantum-Resistant by Construction
- **Active cryptography:** Dilithium-based authentication
- **Hashing:** SHA3 family
- **Future-proof:** Cryptographic primitives may evolve  
  *without changing protocol laws*

---

### 3. Zero-Authority Execution
The Rust layer has:
- No consensus authority
- No cryptographic authority
- No state interpretation
- No time authority

Rust moves bytes.  
Truth exists only in Haskell and formal proofs.

---

## 🛠 High-Level Architecture

The protocol is divided into three immutable layers:

### 🧮 Formal Proof Layer
- Defines protocol invariants and safety laws
- Single source of correctness

### 🧠 Consensus Engine (Haskell)
- Pure, deterministic state transitions
- No IO, no networking, no system time
- Implements Zero-Energy Proof-of-Work (ZeroPoW)

### ⚙️ Execution Shell (Rust)
- `no_std`, memory-safe transport layer
- Stateless FFI boundary
- Mobile-first (Android / Termux capable)
- No protocol authority

---

## 📂 Repository Structure
