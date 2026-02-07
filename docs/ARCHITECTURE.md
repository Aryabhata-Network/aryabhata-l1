# ARYABHATA NETWORK ARCHITECTURE

This document defines the immutable architectural boundaries of the
Aryabhata Layer-1 protocol.

It specifies how protocol laws, formal proofs, consensus logic, and
execution layers interact. These boundaries are fixed after Genesis and
MUST NOT be altered without creating a different protocol.

---

## 1. Architectural Principles

### 1.1 Separation of Truth and Execution

The protocol is divided into three strictly separated concerns:

- **Consensus Truth** — what is valid
- **Execution** — how validity is computed efficiently
- **Formal Proof** — why the rules are correct

No layer may assume the responsibilities of another.

---

### 1.2 Determinism First

Deterministic correctness takes precedence over:

- Performance optimization
- Network latency
- Storage efficiency
- Implementation convenience

Given identical inputs, all compliant nodes MUST reach identical
consensus decisions.

---

### 1.3 Minimal Trust Surface

The architecture assumes a hostile environment.

- No trusted third parties
- No centralized services
- No privileged nodes
- No trusted time, hardware, or network assumptions

All correctness derives solely from protocol laws.

---

### 1.4 Mobile-First Verification

The architecture is designed to support low-resource and mobile devices.

- Verification MUST be possible without specialized hardware
- Consensus correctness MUST be independently verifiable
- Resource constraints MUST NOT affect validity outcomes

Phones are considered first-class verification nodes.

---

### 1.5 Formal Supremacy

Formal mathematical proofs define protocol correctness.

If a conflict exists between:
- source code,
- documentation,
- implementation behavior,
- or social agreement,

the formal proofs are authoritative.

---

## 2. Layered Architecture

### 2.1 Formal Proof Layer (Agda)

**Location:** `proofs/agda/`

**Responsibilities:**
- Define protocol invariants mathematically
- Prove consensus safety and finality properties
- Specify the authoritative correctness model

**Properties:**
- Single source of truth
- Independent of implementation details
- Supersedes all executable code

---

### 2.2 Core Consensus Engine (Haskell)

**Location:** `haskell/src/`

**Responsibilities:**
- Define deterministic block validity rules
- Maintain canonical protocol state
- Enforce all protocol invariants

**Properties:**
- Pure and side-effect free
- No access to networking, storage, or system time
- Produces binary validity decisions only

The consensus engine defines *truth*, not performance.

---

### 2.3 Execution and Networking Layer (Rust)

**Location:** `rust/src/`

**Responsibilities:**
- Peer-to-peer networking
- Transaction and block propagation
- Disk and memory management
- Data transport to the consensus engine

**Properties:**
- Memory-safe execution
- Replaceable without protocol changes
- No authority over consensus outcomes

The execution layer provides *efficiency*, not correctness.

---

### 2.4 FFI Boundary (Critical Security Interface)

**Direction:** Rust → Haskell

**Responsibilities:**
- Pass immutable data structures
- Receive deterministic validation results

**Restrictions:**
- MUST NOT introduce logic
- MUST NOT modify protocol state
- MUST NOT influence consensus decisions

The FFI boundary is a data interface, not a decision layer.

---

## 3. Node Model

Each Aryabhata node operates as an independent verifier.

**Node Characteristics:**
- No trusted peers
- No role-based authority
- No reliance on network majority for correctness

Nodes may differ in performance but MUST agree on validity.

---

## 4. Genesis Configuration

**Location:** `genesis/genesis.json`

**Responsibilities:**
- Define network identity
- Define initial protocol state
- Define immutable parameters

Genesis data contains no logic and no executable behavior.

---

## 5. Architectural Immutability

After Genesis:

- Protocol laws are immutable
- Architectural boundaries are immutable
- Implementations may change
- Optimizations may evolve

Any modification to these architectural boundaries defines a different
protocol, not an upgrade.

---

End of architecture definition.
