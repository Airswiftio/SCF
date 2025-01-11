use crate::errors::Error;
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{panic_with_error, Env};

pub fn has_remainder(env: &Env, id: i128) -> bool {
    let key = DataKey::Remainder(id);
    env.storage().persistent().has(&key)
}

pub fn read_remainder(env: &Env, id: i128) -> i128 {
    let key = DataKey::Remainder(id);
    match env.storage().persistent().get::<DataKey, i128>(&key) {
        Some(target_id) => {
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            target_id
        }
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_remainder(env: &Env, id: i128, target_id: i128) {
    let key = DataKey::Remainder(id);
    env.storage().persistent().set(&key, &target_id);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
