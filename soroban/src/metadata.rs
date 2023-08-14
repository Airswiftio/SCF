use crate::errors::Error;
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT};
use soroban_sdk::{panic_with_error, Address, Env, String, Symbol};

pub fn read_expired(env: &Env) -> bool {
    let key = DataKey::Expired;
    match env.storage().persistent().get::<DataKey, bool>(&key) {
        Some(data) => {
            env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
            data
        }
        None => false,
    }
}

pub fn write_expired(env: &Env, val: bool) {
    let key = DataKey::Expired;
    env.storage().persistent().set(&key, &val);
    env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
}

pub fn read_paid(env: &Env) -> bool {
    let key = DataKey::Paid;
    match env.storage().persistent().get::<DataKey, bool>(&key) {
        Some(data) => {
            env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
            data
        }
        None => false,
    }
}

pub fn write_paid(env: &Env, val: bool) {
    let key = DataKey::Paid;
    env.storage().persistent().set(&key, &val);
    env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
}

pub fn read_external_token_provider(env: &Env) -> Address {
    let key = DataKey::ExternalTokenProvider;
    match env.storage().persistent().get::<DataKey, Address>(&key) {
        Some(data) => {
            env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
            data
        }
        None => panic_with_error!(env, Error::InvalidContract),
    }
}

pub fn write_external_token_provider(env: &Env, addr: Address) {
    let key = DataKey::ExternalTokenProvider;
    env.storage().persistent().set(&key, &addr);
    env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
}
