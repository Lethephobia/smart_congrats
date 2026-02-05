# SmartCongrats

An upcoming desktop app to celebrate any “special day” on-chain (not limited to birthdays).

## MVP Goals

- Use Solana (Anchor) for `Day` creation and `Celebrate` records (log-first)
- Flutter desktop (macOS → Windows) for create/celebrate/history UI
- Use v0 (Versioned Transaction) from the start
- Keep Dart thin by pushing Solana complexity into Rust (FFI)

## Repository Layout

- `onchain/`: Solana / Anchor program (generated later via `anchor init`)
- `sdk/`: Rust SDK workspace (layered; FRB boundary in `crates/presentation`)
- `apps/`: User-facing apps (Flutter app, CLI, etc.)
- `docs/`: Specs and design notes

## Next Steps

1. Fill the open questions in `docs/mvp.md` (PDA uniqueness, date encoding, log vs storage, etc.)
2. Implement the on-chain program in `onchain/programs/smart_congrats/` and shared logic in `onchain/crates/*`
3. Implement the client-side SDK in `sdk/` (FRB boundary in `sdk/crates/presentation/`)
4. Run `flutter pub get` in `apps/smart_congrats_app/`, then run the Flutter app from `apps/smart_congrats_app/packages/presentation/`
