# Dev Setup (Draft)

This repository is split into three areas:

- `onchain/` (Anchor / Solana)
- `sdk/` (Rust SDK + FRB boundary)
- `apps/` (Flutter app + CLI, etc.)

## Prerequisites (Typical)

- Rust toolchain (stable)
- Solana CLI (for devnet/mainnet configuration)
- Anchor (program development)
- Flutter (desktop enabled)

## Local Dev Flow (Planned)

1. Finalize `docs/mvp.md` (close the open questions)
2. Build and iterate on the Anchor workspace in `onchain/`
3. Implement the Rust SDK in `sdk/` and expose a minimal FRB API
4. Run `flutter pub get` in `apps/smart_congrats_app/`, then run the Flutter app in `apps/smart_congrats_app/packages/presentation/` and call `sdk/` via FFI

Concrete commands will be added after the Anchor / Flutter scaffolding is generated.
