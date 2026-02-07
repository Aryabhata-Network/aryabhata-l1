# PROTOCOL INVARIANTS
## Aryabhata Layer-1 (Immutable Core Laws)

These invariants define the supreme, immutable laws of the Aryabhata
Layer-1 protocol.

After Genesis, no human, organization, AI system, governance process,
or future implementation has authority to alter these laws.

Any system violating these invariants is NOT a valid Aryabhata network.
Formal correctness is defined by mathematical proofs in `proofs/agda/`.

---

## 1. Supply Invariance
**Law:** Total supply is finite and constant after Genesis.
**Enforcement:** Any block that changes total supply is invalid.
**Proof:** `proofs/agda/ProtocolLaws.agda` — SupplyInvariant

## 2. Deterministic Consensus
**Law:** Validity is a pure deterministic function of previous state,
block data, and immutable parameters.
**Enforcement:** Identical inputs ⇒ identical results.
**Proof:** `proofs/agda/ConsensusSafety.agda` — Determinism

## 3. Post-Quantum Authentication
**Law:** Authentication relies only on post-quantum hardness assumptions.
Classical (RSA/ECDSA/ECC) schemes are forbidden.
**Enforcement:** Non-PQ authentication ⇒ invalid block.
**Proof:** `proofs/agda/AdversaryModel.agda` — QuantumAdversary

## 4. Finality Irreversibility
**Law:** Once protocol-defined finality is satisfied, a block is irreversible.
**Enforcement:** Reorgs violating finality are rejected.
**Proof:** `proofs/agda/ConsensusSafety.agda` — FinalityMonotonicity

## 5. No Privileged Authority
**Law:** No admin keys, governance, emergency overrides, or upgrade switches.
**Enforcement:** Any privileged rule-change path is invalid.
**Proof:** `proofs/agda/ProtocolLaws.agda` — NoAuthority

## 6. Complete State Definition
**Law:** All state is fully determined by consensus rules only.
**Enforcement:** Non-derivable state is invalid.
**Proof:** `proofs/agda/ProtocolLaws.agda` — StateCompleteness

## 7. Implementation Independence
**Law:** Consensus truth is independent of language, compiler, OS, or hardware.
**Enforcement:** Divergent behavior ⇒ non-compliant.
**Proof:** `proofs/agda/ProtocolLaws.agda` — ImplementationIndependence

## 8. FFI Boundary Isolation
**Law:** FFI passes data only; never logic or decisions.
**Enforcement:** Any FFI-influenced decision is invalid.
**Proof:** `proofs/agda/FFIBoundary.agda` — Isolation

## 9. Resource Neutrality
**Law:** Validity does not depend on device resources or location.
**Enforcement:** Resource-dependent checks are ignored.
**Proof:** `proofs/agda/AdversaryModel.agda` — ResourceIndependence

## 10. Time Independence
**Law:** After Genesis, validity is independent of wall-clock time.
**Enforcement:** Time-based validity checks are forbidden.
**Proof:** `proofs/agda/ProtocolLaws.agda` — TimeIndependence

## 11. Mathematical Supremacy
**Law:** Formal proofs define correctness.
**Enforcement:** Code contradicting proofs is invalid.
**Proof:** `proofs/agda/ProtocolLaws.agda` — ProofSupremacy

## 12. Invariant Supremacy
**Law:** These invariants supersede all code, docs, and social agreements.
