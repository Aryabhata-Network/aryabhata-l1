# Arya-Vault — Wallet Specification

## Key Generation
entropy (256 bits)
-> Argon2id (memory: 256MB, iterations: 4)
-> seed
-> Dilithium5 keypair
Seed phrase: 24 words (BIP-39)
Displayed exactly once at creation.
Never stored on device.
Never sent to any server.

---

## Recovery Options

| Method | Description |
|---|---|
| Seed phrase | 24 words — user responsibility |
| Shamir Secret Sharing | Split into N shares, requires M to recover |
| Social Recovery | Up to 5 guardians, 3-of-5 required |
| Protocol recovery | None — protocol holds zero keys |

---

## Shamir Secret Sharing

User chooses N total shares and M required shares.
Example: 3-of-5 means any 3 of 5 shares recovers wallet.
User distributes shares to trusted locations.
Protocol does not hold any share.

---

## Social Recovery

User designates up to 5 guardian addresses.
Recovery requires 3-of-5 guardian consensus.
Recovery transaction is time-locked for 72 hours.
Sender can cancel within first 36 hours.

---

## Time-Lock Policy

Large transfers above 100,000 ARY:
- On-chain but unconfirmed for 24 hours
- Sender can cancel within first 12 hours
- Provides defence against theft
- No centralised intervention possible

---

## Security Properties

- Argon2id — memory-hard, GPU-resistant key derivation
- Dilithium5 — quantum-resistant signatures
- Key material zeroized from memory after each use
- No seed phrase stored anywhere on device
- No server backup — ever

---

*Version 2.0 — 2025 — Aryabhata Network*
