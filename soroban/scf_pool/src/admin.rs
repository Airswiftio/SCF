use soroban_sdk::{Address, Env};

use crate::storage_types::{DataKey, TokenInfo};

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn get_pool_token(e: &Env) -> TokenInfo {
    let key = DataKey::PoolToken;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_pool_token(e: &Env, token: &TokenInfo) {
    let key = DataKey::PoolToken;
    e.storage().instance().set(&key, token);
}
