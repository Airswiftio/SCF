use crate::errors::Error;
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT};
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_owner(env: &Env, id: i128) -> Address {
    let key = DataKey::Owner(id);
    match env.storage().persistent().get::<DataKey, Address>(&key) {
        Some(balance) => {
            env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
            balance
        }
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_owner(env: &Env, id: i128, owner: Option<Address>) {
    let key = DataKey::Owner(id);
    env.storage().persistent().set(&key, &owner);
    env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
}

pub fn check_owner(env: &Env, auth: &Address, id: i128) {
    if auth != &read_owner(env, id) {
        panic_with_error!(env, Error::NotOwned)
    }
}
