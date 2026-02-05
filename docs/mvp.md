# MVP Spec (Draft)

## Open Questions (Must Decide)

- `Day` uniqueness (PDA seeds)
- Date representation (encoding for YYYY-MM-DD, timezone policy)
- `Celebrate`: logs-only vs storing records on-chain
- Message: plaintext on-chain vs hash-only
- Permissions (who can create a Day, who can celebrate)

## Proposed Minimal Defaults

- Make `Day` unique via a PDA
- Make `Celebrate` log-first via `emit!` (history displayed by fetching program logs)
- Prefer message hashes (consider plaintext later if needed)
