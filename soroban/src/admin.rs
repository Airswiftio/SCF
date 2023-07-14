use crate::storage_types::DataKey;
use soroban_sdk::{Env, Address};

pub fn has_administrator(env: &Env) -> bool {
    let key = DataKey::Admin;
    env.storage().has(&key)
}

pub fn read_administrator(env: &Env) -> Address {
    let key = DataKey::Admin;
    env.storage().get_unchecked(&key).unwrap()
}

pub fn write_administrator(env: &Env, id: &Address) {
    let key = DataKey::Admin;
    env.storage().set(&key, id);
}