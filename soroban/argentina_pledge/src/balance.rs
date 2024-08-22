use crate::storage_types::DataKey;
use soroban_sdk::Env;

pub fn read_supply(e: &Env) -> u64 {
    let key = DataKey::Supply;
    match e.storage().instance().get::<DataKey, u64>(&key) {
        Some(balance) => balance,
        None => 0,
    }
}

pub fn increment_supply(e: &Env) {
    let key = DataKey::Supply;
    e.storage().instance().set(&key, &(read_supply(&e) + 1));
}
