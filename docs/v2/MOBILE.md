# Mobile-First Mining — Termux Guide

## Why Mobile-First

6.8 billion smartphones exist globally.
Full participation accessible to anyone with a phone.
Minimum hardware cost: zero for existing smartphone owners.

---

## Hardware Requirements

| Requirement | Minimum |
|---|---|
| OS | Android 8.0 or higher |
| Architecture | AArch64 (ARM64) |
| RAM | 512 MB free |
| Storage | 256 MB free |
| CPU | Any ARM64 processor |

---

## Installation

```bash
pkg update && pkg upgrade -y
pkg install -y rust clang git
git clone https://github.com/Aryabhata-Network/aryabhata-l1.git
cd aryabhata-l1/rust
cargo build --release
Running the Node
Standard mode:
./target/release/aryabhata-node --mode=pruned
Low power mode (recommended for mobile):
./target/release/aryabhata-node --mode=pruned --mining=low-power
Mobile Optimisations
Feature
Value
Binary size
Less than 15 MB
RAM idle
Less than 40 MB
RAM mining
Less than 120 MB
Storage pruned
Less than 200 MB
Thermal limit
Pauses at 42 degrees Celsius
Low power mode
Reduces hash rate, saves battery
Low Power Mode
Reduces hash rate by 70 percent.
Extends battery life approximately 3 times.
Still fully participates in consensus.
Recommended for daily use on mobile.
Pruned Mode
Stores only block headers and UTXO set.
Discards full block data after confirmation.
Total storage under 200 MB.
Full security — no trust required.
Version 2.0 — 2025 — Aryabhata Network
