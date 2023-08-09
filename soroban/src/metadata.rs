use crate::errors::Error;
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT};
use soroban_sdk::{panic_with_error, Env, String, Symbol};

pub fn read_name(env: &Env) -> String {
    let key = DataKey::Name;
    match env.storage().instance().get::<DataKey, String>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_name(env: &Env, name: &String) {
    let key = DataKey::Name;
    env.storage().instance().set(&key, name)
}

pub fn read_symbol(env: &Env) -> Symbol {
    let key = DataKey::Symbol;
    match env.storage().instance().get::<DataKey, Symbol>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_symbol(env: &Env, symbol: &Symbol) {
    let key = DataKey::Symbol;
    env.storage().instance().set(&key, symbol)
}

pub fn read_token_uri(env: &Env, id: i128) -> String {
    let key = DataKey::URI(id);
    match env.storage().persistent().get::<DataKey, String>(&key) {
        Some(data) => {
            env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
            data
        }
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_token_uri(env: &Env, id: i128, uri: String) {
    let key = DataKey::URI(id);
    env.storage().persistent().set(&key, &uri);
    env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
}

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
