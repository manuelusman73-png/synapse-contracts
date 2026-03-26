use crate::types::{DlqEntry, Settlement, Transaction};
use soroban_sdk::{contracttype, Address, Env, String as SorobanString};

// TODO(#58): bump TTL on every persistent read (extend_ttl) to prevent state expiry
// TODO(#59): [DONE] added TempLock + lock_temp helpers
// TODO(#60): add DlqCount key to track total DLQ entries without scanning

const TX_TTL_THRESHOLD: u32 = 17_280;
const TX_TTL_EXTEND_TO: u32 = 172_800;

#[contracttype]
pub enum StorageKey {
    Admin,
    Paused,
    MinDeposit,
    MaxDeposit,
    Relayer(Address),
    Asset(SorobanString),
    Tx(SorobanString),
    AnchorIdx(SorobanString),
    Settlement(SorobanString),
    Dlq(SorobanString),
    TempLock(SorobanString),
}

pub mod admin {
    use super::*;
    pub fn set(env: &Env, admin: &Address) {
        env.storage().instance().set(&StorageKey::Admin, admin);
    }
    pub fn get(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&StorageKey::Admin)
            .expect("not initialised")
    }
}

pub mod pause {
    use super::*;
    // TODO(#61): check paused state at the top of every mutating function
    pub fn set(env: &Env, paused: bool) {
        env.storage().instance().set(&StorageKey::Paused, &paused);
    }
    pub fn is_paused(env: &Env) -> bool {
        env.storage()
            .instance()
            .get(&StorageKey::Paused)
            .unwrap_or(false)
    }
}

pub mod relayers {
    use super::*;
    pub fn add(env: &Env, r: &Address) {
        env.storage()
            .instance()
            .set(&StorageKey::Relayer(r.clone()), &true);
    }
    pub fn remove(env: &Env, r: &Address) {
        env.storage()
            .instance()
            .remove(&StorageKey::Relayer(r.clone()));
    }
    pub fn has(env: &Env, r: &Address) -> bool {
        env.storage()
            .instance()
            .has(&StorageKey::Relayer(r.clone()))
    }
}

pub mod assets {
    use super::*;
    pub fn add(env: &Env, code: &SorobanString) {
        if is_allowed(env, code) {
            return;
        }
        env.storage()
            .instance()
            .set(&StorageKey::Asset(code.clone()), &true);
    }
    pub fn remove(env: &Env, code: &SorobanString) {
        if !is_allowed(env, code) {
            return;
        }
        env.storage()
            .instance()
            .remove(&StorageKey::Asset(code.clone()));
    }
    pub fn is_allowed(env: &Env, code: &SorobanString) -> bool {
        env.storage()
            .instance()
            .has(&StorageKey::Asset(code.clone()))
    }
    pub fn require_allowed(env: &Env, code: &SorobanString) {
        if !is_allowed(env, code) {
            panic!("asset not allowed")
        }
    }
}



pub mod deposits {
    use super::*;
    pub fn save(env: &Env, tx: &Transaction) {
        let key = StorageKey::Tx(tx.id.clone());
        env.storage().persistent().set(&key, tx);
        env.storage()
            .persistent()
            .extend_ttl(&key, TX_TTL_THRESHOLD, TX_TTL_EXTEND_TO);
    }
    pub fn get(env: &Env, id: &SorobanString) -> Transaction {
        env.storage()
            .persistent()
            .get(&StorageKey::Tx(id.clone()))
            .expect("tx not found")
    }
    pub fn index_anchor_id(env: &Env, anchor_id: &SorobanString, tx_id: &SorobanString) {
        env.storage()
            .persistent()
            .set(&StorageKey::AnchorIdx(anchor_id.clone()), tx_id);
    }
    pub fn find_by_anchor_id(env: &Env, anchor_id: &SorobanString) -> Option<SorobanString> {
        env.storage()
            .persistent()
            .get(&StorageKey::AnchorIdx(anchor_id.clone()))
    }
}

pub mod settlements {
    use super::*;
    pub fn save(env: &Env, s: &Settlement) {
        env.storage()
            .persistent()
            .set(&StorageKey::Settlement(s.id.clone()), s);
    }
    pub fn get(env: &Env, id: &SorobanString) -> Settlement {
        env.storage()
            .persistent()
            .get(&StorageKey::Settlement(id.clone()))
            .expect("settlement not found")
    }
    pub fn extend_ttl(env: &Env, id: &SorobanString) {
        env.storage().persistent().extend_ttl(&StorageKey::Settlement(id.clone()), 535679, 535679);
    }
}

pub mod max_deposit {
    use super::*;
    pub fn set(env: &Env, amount: i128) {
        env.storage().instance().set(&StorageKey::MaxDeposit, &amount);
    }
    pub fn get(env: &Env) -> Option<i128> {
        env.storage().instance().get(&StorageKey::MaxDeposit)
    }
}

pub mod dlq {
    use super::*;
    pub fn push(env: &Env, entry: &DlqEntry) {
        env.storage()
            .persistent()
            .set(&StorageKey::Dlq(entry.tx_id.clone()), entry);
    }
    pub fn get(env: &Env, tx_id: &SorobanString) -> Option<DlqEntry> {
        env.storage()
            .persistent()
            .get(&StorageKey::Dlq(tx_id.clone()))
    }
    pub fn remove(env: &Env, tx_id: &SorobanString) {
        // TODO(#62): call this after a successful retry
        env.storage()
            .persistent()
            .remove(&StorageKey::Dlq(tx_id.clone()));
    }
}

pub mod temp_lock {
    use super::*;

    const TEMP_LOCK_THRESHOLD: u32 = 3600;
    const TEMP_LOCK_EXTEND_TO: u32 = 7200;

    pub fn lock(env: &Env, key: &SorobanString) {
        let lock_key = StorageKey::TempLock(key.clone());
        if env.storage().temporary().has(&lock_key) {
            panic!("idempotency lock active");
        }
        env.storage().temporary().set(&lock_key, &true);
        env.storage()
            .temporary()
            .extend_ttl(&lock_key, TEMP_LOCK_THRESHOLD, TEMP_LOCK_EXTEND_TO);
    }

    pub fn unlock(env: &Env, key: &SorobanString) {
        let lock_key = StorageKey::TempLock(key.clone());
        env.storage().temporary().remove(&lock_key);
    }

    pub fn is_locked(env: &Env, key: &SorobanString) -> bool {
        env.storage().temporary().has(&StorageKey::TempLock(key.clone()))
    }
}

pub use temp_lock::{lock as lock_temp, unlock as unlock_temp, is_locked as is_temp_locked};

