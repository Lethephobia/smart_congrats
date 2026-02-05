# Architecture (Draft)

## Components

- On-chain: Solana / Anchor (`Day` and `Celebrate`)
- SDK: Rust (called from Flutter via FFI)
  - Build v0 transactions
  - Sign
  - Send via RPC (with retries)
- App: Flutter desktop (UX-first, keep Dart thin)

## Key Management (Policy)

- Do not require external wallet integrations; handle keys in-app (beginner-friendly)
- Derive from BIP39 mnemonic using `m/44'/501'/0'/0'`
- Persist secrets using Flutter-side OS secure storage (e.g., macOS Keychain)
- For MVP, allow Rust to sign and send end-to-end (Flutter passes mnemonic/seed per call)
