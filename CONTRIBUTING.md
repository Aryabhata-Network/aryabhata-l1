# Contributing to Aryabhata

## Philosophy

Aryabhata is a discipline-driven protocol.
Contributions must meet the same standard.

---

## Rules

**Code rules:**
- No `unsafe` in Rust — ever
- No `IO` in Haskell consensus — ever
- No classical cryptography — ever
- All PRs must include tests
- All tests must pass

**Commit rules:**
- One logical change per commit
- Clear commit message
- No merge commits on main

**Review rules:**
- All PRs require review before merge
- Security-related PRs require two reviews
- No self-merge

---

## Development Setup

```bash
# Rust
cd rust
cargo build
cargo test

# Haskell
cd haskell
cabal build
cabal test
What we accept
Bug fixes with test coverage
Performance improvements
Security improvements
Documentation improvements
What we do not accept
Protocol rule changes
Consensus changes
Supply changes
Cryptography downgrades
Aryabhata Network — 2025
