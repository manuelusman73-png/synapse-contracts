# TODO Progress for Issue #59: temporary idempotency locks

## Plan Steps:
1. [x] Checkout to branch `feature/issue-59-temp-idem-locks` from develop
2. [x] Add TempLock key & helpers to src/storage/mod.rs
3. [x] Update register_deposit in src/lib.rs to use lock_temp(anchor_id)
4. [] Update mark_* functions in src/lib.rs to use lock_temp(tx_id)
5. [] Update finalize_settlement in src/lib.rs to use lock_temp(settlement_id)
6. [] Run `cargo test`, update snapshots
7. [] Commit changes
8. [] Push branch
9. [] Create PR to develop

Current: Completed step 1.
