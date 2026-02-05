# sdk

Rust workspace for the SmartCongrats client-side SDK used by:
- the Flutter app via `flutter_rust_bridge` (FRB)
- a future CLI (planned)

This workspace follows a layered architecture:

- `crates/domain`: value objects, invariants, pure logic (no FRB dependency)
- `crates/application`: use cases and orchestration (no FRB dependency)
- `crates/infrastructure`: RPC, transaction sending, and other adapters (no FRB dependency)
- `crates/presentation`: FRB-facing API surface (thin boundary layer; calls into other crates)

Dependency direction (one-way):
`presentation -> application -> domain`
and
`presentation/application -> infrastructure`

Notes:
- Keep `presentation` minimal: only Dart-friendly DTOs and functions exported over FRB.
- Keep on-chain/program specifics out of this SDK where possible; prefer generic Solana primitives.

