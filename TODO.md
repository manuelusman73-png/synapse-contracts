# Task: Fix DLQ Retry, Asset Cap, and Wire Guards

Approved plan to fix snapshot failures: full DLQ retry logic (#29,#31,#62), asset allowlist cap (#13), wire require_not_paused (#63). Branch: `feature/fix-dlq-asset-cap`.

## Steps to Complete:
- [ ] 1. Create/update branch: `git checkout -b feature/fix-dlq-asset-cap`
- [ ] 2. Edit src/types/mod.rs: Add Event::DlqRetried(SorobanString), Event::MaxRetriesExceeded(SorobanString); remove related TODOs.
- [ ] 3. Edit src/storage/mod.rs: Add asset_count storage (get/set/inc/dec); enforce MAX_ASSETS=50u32; fix assets::remove duplicate.
- [ ] 4. Edit src/lib.rs: Implement asset cap in add_asset (panic if exceeded); full retry_dlq (max retries check, remove DLQ on success, emit events); wire require_not_paused guards; fix duplicate set_max_deposit; add missing events (e.g., RelayerGranted).
- [ ] 5. Edit tests/contract_test.rs: Add/update tests for new logic (asset cap exceeds/respects, retry removes DLQ, max retries).
- [ ] 6. Run `cargo test` to verify snapshots pass.
- [ ] 7. Update this TODO.md with completions.
- [ ] 8. Commit changes, push, create PR to develop.

