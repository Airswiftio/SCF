use soroban_sdk::{panic_with_error, Address, Env};

use crate::{
    errors::Error,
    storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD},
};

pub fn write_owner(e: &Env, id: u64, owner: Option<Address>) {
    let key = DataKey::Owner(id);
    match owner {
        Some(owner) => {
            e.storage().persistent().set(&key, &owner);
            e.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
        }
        None => e.storage().persistent().remove(&key),
    }
}

pub fn read_owner(e: &Env, id: u64) -> Address {
    let key = DataKey::Owner(id);
    match e.storage().persistent().get::<DataKey, Address>(&key) {
        Some(owner) => {
            e.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            owner
        }
        None => panic_with_error!(&e, Error::NotFound),
    }
}

pub fn check_owner(e: &Env, auth: &Address, id: u64) {
    if auth != &read_owner(e, id) {
        panic_with_error!(e, Error::NotOwned)
    }
}
