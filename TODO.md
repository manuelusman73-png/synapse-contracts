# Feature/issue-62-extend-ttl Progress

## Remaining Steps
- [x] 1. Add TTL helper functions (extend_persistent_ttl, extend_instance_ttl) to src/storage/mod.rs
- [x] 2. Update all target get functions to call extend_ttl after read
- [x] 3. Remove TODO(#58) and settlements::extend_ttl hardcoded function
- [ ] 4. Add test functions for TTL extension in tests/contract_test.rs
- [ ] 5. Run `cargo test` to generate snapshots and verify
- [ ] 6. Commit changes and open PR to develop
- [ ] 4. Add test functions for TTL extension in tests/contract_test.rs
- [ ] 5. Run `cargo test` to generate snapshots and verify
- [ ] 6. Commit changes and open PR to develop

Updated as steps complete.
