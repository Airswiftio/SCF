use crate::storage_types::DataKey;
use crate::errors::{Error};
use soroban_sdk::{panic_with_error, Env, Address};

pub fn read_owner(env: &Env, id: i128) -> Address {
    let key = DataKey::Owner(id);
    match env.storage().get(&key) {
        Some(balance) => balance.unwrap(),
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_owner(env: &Env, id: i128, owner: Option<Address>) {
    let key = DataKey::Owner(id);
    env.storage().set(&key, &owner);
}

pub fn check_owner(env: &Env, auth: &Address, id: i128) {
    if auth != &read_owner(env, id) {
        panic_with_error!(env, Error::NotOwned)
    }
}